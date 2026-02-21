use ircv3_parse::Message;
use rand::distr::uniform::SampleRange;
use std::ops::RangeInclusive;
use std::time::Duration;
use std::{marker::PhantomData, net::SocketAddr, pin::Pin};
use tokio::io::{
    AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, ReadHalf, WriteHalf
};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::server::TlsStream;

use crate::irc::error::{IrcResult, IrcSessionError};
use crate::tls::TlsHandler;

mod capability;
mod command;
mod error;
mod ipc;
mod session;
pub use session::*;

pub type NetSource = BufReader<ReadHalf<TlsStream<TcpStream>>>;
pub type NetSink = WriteHalf<TlsStream<TcpStream>>;

#[derive(Debug, Clone, Default)]
pub struct IrcServer {
    // Do not allow this struct to be constructed
    // directly, force users to construct it using
    // the Default constructor or else use a future
    // API method.
    _ne: PhantomData<()>,
}

impl TlsHandler for IrcServer {
    type Future = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;

    fn handle(&mut self, stream: TlsStream<TcpStream>, client_addr: SocketAddr) -> Self::Future {
        Box::pin(IrcConnection::new(stream, client_addr).run())
    }
}

pub struct IrcConnection {
    #[allow(unused)] //placeholder
    sys: (),

    /// Abstract byte stream from one client only.
    rx: NetSource,

    /// Abstract byte stream to the same client
    /// sending through rx.
    tx: NetSink,

    /// The IPv4/IPv6 address of the remote client.
    client_addr: SocketAddr,
}

impl IrcConnection {
    // Clients that don't send anything before this random window
    // are considered idle.
    const CONN_MSG_TIMEOUT_MS: RangeInclusive<u64> = 8000..=15000;

    pub fn new(stream: TlsStream<TcpStream>, client_addr: SocketAddr) -> Self {
        let (rx, tx) = tokio::io::split(stream);
        Self {
            sys: (),
            rx: BufReader::new(rx),
            tx,
            client_addr,
        }
    }

    /// Launch the connection listener daemon-style.
    async fn run(mut self) {
        // Buffer to store message contents. Parsed messages borrow
        // from this buffer instead of cloning data.
        let mut rx_buf = Vec::new();

        let mut session = IrcSession::new(self.client_addr);
        let mut state = IrcState::default();

        loop {
            rx_buf.clear();

            // Get the next message from the client, or else time out.
            // Random timeout between 8s and 15s to prevent a thundering
            // herd all at once.
            let next = self
                .rx
                .next_message(
                    &mut rx_buf,
                    Self::CONN_MSG_TIMEOUT_MS,
                    8192, // TODO
                )
                .await;

            let msg = match next {
                Ok(Some(m)) => m,
                Ok(None) => match session.ping_keepalive(&mut self.tx).await {
                    Ok(_) => continue,
                    Err(_e) => break, // TODO better error handling
                }
                Err(_e) => break, // TODO better error handling
            };

            #[cfg(debug_assertions)]
            println!(">>> {msg}");

            let res = match state {
                IrcState::Anonymous(s) => {
                    IrcContext::new(s, &mut session, &mut self.tx).execute(&msg).await
                }
                IrcState::Registered(s) => {
                    IrcContext::new(s, &mut session, &mut self.tx).execute(&msg).await
                }
                IrcState::Authenticated(s) => {
                    IrcContext::new(s, &mut session, &mut self.tx).execute(&msg).await
                }
            };

            match res {
                Ok(new_state) => state = new_state,
                Err(_e) => break, // TODO better error handling
            }
        }

        self.close_connection().await;
    }

    /// Attempt to gracefully close the connection by issuing a TLS shutdown
    /// and closing the socket.
    async fn close_connection(self) {
        if let Err(e) = self.rx.into_inner().unsplit(self.tx).shutdown().await {
            eprintln!(
                "Failed to gracefully close connection with {}: {e}",
                self.client_addr
            );
        }
    }
}

enum ReadLineResult {
    Line,    // Successfully read a line ending with \r\n
    TooLong, // Hit max length without finding \r\n
    Eof,     // Connection closed
}

trait MessageExt {
    /// Parse a `Message` out of the given buffer. The Message
    /// borrows data from the buffer, so the buffer must live
    /// as long as the message.
    #[inline(always)]
    fn from_raw<'a>(buf: &'a [u8]) -> IrcResult<Message<'a>> {
        // Convert to string, removing \r\n
        let line = std::str::from_utf8(buf)
            .unwrap_or("<invalid utf8>")
            .trim_end_matches("\r\n");

        Ok(ircv3_parse::parse(line)?)
    }
}
impl MessageExt for Message<'_> {}

trait IrcMessageReaderExt {
    /// Consume from `self` until wither a full UTF-8 string delimited by `\r\n`
    /// is read, or return with an error if the line is malformed oro too long.
    async fn read_line_limited(
        &mut self,
        buf: &mut Vec<u8>,
        max_line_length: u64,
    ) -> std::io::Result<ReadLineResult>;

    /// Read the next IRC message from the connection, with a randomized timeout to avoid
    /// synchronized time-out cascades across connections.
    ///
    /// - `buffer`: mutable byte buffer reused to store the incoming line.
    /// - Timeout: randomized between 8_000 and 15_000 ms to mitigate thundering-herd effects.
    /// - Returns `Ok(Some(message))` when a client message is ready.
    /// - Returns `Ok(None)` when the client timeout hits.
    /// - Returns `Err(IrcSessionError)` if there is a transport or
    ///   parsing error.
    async fn next_message<'a>(
        &mut self,
        buf: &'a mut Vec<u8>,
        timeout_ms: impl SampleRange<u64>,
        max_line_length: u64,
    ) -> IrcResult<Option<Message<'a>>>;
}

impl<R> IrcMessageReaderExt for R
where
    R: AsyncBufRead + Unpin,
{
    async fn read_line_limited(
        &mut self,
        buf: &mut Vec<u8>,
        max_line_length: u64,
    ) -> std::io::Result<ReadLineResult> {
        let bytes_read = self.take(max_line_length).read_until(b'\n', buf).await?;

        Ok(match (bytes_read, buf.as_slice()) {
            (0, _) => ReadLineResult::Eof,
            (_, b) if b.ends_with(b"\r\n") => ReadLineResult::Line,
            (_, b) if b.len() >= max_line_length as usize => ReadLineResult::TooLong,
            _ => ReadLineResult::TooLong, // Reject malformed lines
        })
    }

    async fn next_message<'a>(
        &mut self,
        buf: &'a mut Vec<u8>,
        timeout_ms: impl SampleRange<u64>,
        max_line_length: u64,
    ) -> IrcResult<Option<Message<'a>>> {
        let t = timeout(
            Duration::from_millis(rand::random_range(timeout_ms)),
            self.read_line_limited(buf, max_line_length),
        )
        .await;

        match t {
            // Timed out
            Err(_) => Ok(None),
            // Either a line or an error
            Ok(r) => match r? {
                ReadLineResult::Line => Message::from_raw(buf).map(Some),
                ReadLineResult::TooLong => Err(IrcSessionError::MessageTooLong),
                ReadLineResult::Eof => Err(IrcSessionError::ClientEOF),
            },
        }
    }
}

trait IrcMessageWriterExt {
    async fn send_unchecked(&mut self, msg: &(impl AsRef<[u8]> + ?Sized)) -> IrcResult<()>;
}

impl<T> IrcMessageWriterExt for T where T: AsyncWrite + Unpin {
    async fn send_unchecked(&mut self, msg: &(impl AsRef<[u8]> + ?Sized)) -> IrcResult<()> {
        #[cfg(debug_assertions)]
        println!(
            "<<< {}",
            str::from_utf8(msg.as_ref()).unwrap_or("<invalid utf8>")
        );

        self.write_all(msg.as_ref()).await?;
        self.flush().await?;
        Ok(())
    }
}