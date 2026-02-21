use std::{net::SocketAddr, pin::Pin, sync::Arc, time::Duration};

use bytes::Bytes;
use ircv3_parse::Message;
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, split},
    select,
    sync::broadcast,
    time::{Instant, Sleep, sleep},
};
use tokio_stream::{StreamExt, StreamMap};

use crate::{
    error::{IrcResult, IrcSessionError},
    irc::{
        ChannelName, ChannelSink, ChannelSource, ClientSink, ClientSource, IrcContext, IrcSession, ServerMessage, ServerSink, ServerSource, command, state
    },
    storage::Storage,
    tls::TlsHandler,
};

pub struct IrcServer<S> {
    storage: Arc<S>,
    s_rx: broadcast::Receiver<()>,
    s_tx: broadcast::Sender<()>,
}

impl<S> IrcServer<S> {
    pub fn new(storage: S) -> Self {
        let (s_tx, s_rx) = broadcast::channel(1024);
        let storage = Arc::new(storage);

        Self {
            storage,
            s_rx,
            s_tx,
        }
    }
}

impl<S> TlsHandler for IrcServer<S>
where
    S: Storage + 'static,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;

    fn handle(
        &mut self,
        stream: tokio_rustls::server::TlsStream<tokio::net::TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future {
        let (r_rx, r_tx) = split(stream);
        let (s_rx, s_tx) = (self.s_tx.subscribe(), self.s_tx.clone().downgrade());

        Box::pin(
            IrcConnection {
                storage: Arc::clone(&self.storage),

                client_addr,

                r_rx: BufReader::new(r_rx),
                r_tx,

                s_rx,
                s_tx,

                c_tx: StreamMap::new(),
                c_rx: StreamMap::new(),

                timeout: Box::pin(sleep(Duration::from_secs(30))),
            }
            .run(),
        )
    }
}

pub struct IrcConnection<S> {
    // Storage backend
    storage: Arc<S>,

    // Remote host address
    client_addr: SocketAddr,

    // I/O with the client this session is associated with.
    r_rx: ClientSource,
    r_tx: ClientSink,

    // Spurious I/O from the server.
    s_rx: ServerSource,
    s_tx: ServerSink,

    // Channels the user is joined to.
    c_rx: StreamMap<ChannelName, ChannelSource>,
    c_tx: StreamMap<ChannelName, ChannelSink>,

    // Inactivity timeout counter
    timeout: Pin<Box<Sleep>>,
}

impl<S> IrcConnection<S>
where
    S: Storage,
{
    pub async fn run(mut self) {
        // Stores data that must be read in from our owned-handle (i.e. Client rx)
        let mut own_buf = Vec::<u8>::new();

        // Stores data shared over pipe
        let mut ref_buf = Arc::new(Bytes::new());

        let mut session = IrcSession::default();
        let mut state = state::Anonymous::default();

        let e: IrcResult<()> = loop {
            let next = self.next_incoming(&mut own_buf, &mut ref_buf).await;
            match next {
                Ok(Signal::Timeout) => {
                    let mut ctx = IrcContext::new(
                        &mut session,
                        &mut self.r_tx,
                        &mut self.s_tx,
                        &mut self.c_tx,
                        state,
                    );

                    ctx.ping_keepalive().await.unwrap(); // TODO better error handling
                    state = ctx.into();
                }
                Ok(Signal::Client(msg)) => {
                    let ctx =IrcContext::new(
                        &mut session,
                        &mut self.r_tx,
                        &mut self.s_tx,
                        &mut self.c_tx,
                        state,
                    );

                    // TODO better error handling
                    let ctx = command::_route(ctx, msg).await.unwrap(); 
                    state = ctx.into();
                }
                Ok(Signal::Server(msg)) => {
                    let ctx = IrcContext::new(
                        &mut session,
                        &mut self.r_tx,
                        &mut self.s_tx,
                        &mut self.c_tx,
                        state,
                    );

                    state = ctx.into();

                }
                Ok(Signal::Channel(_name, msg)) => {
                    let mut ctx = IrcContext::new(
                        &mut session,
                        &mut self.r_tx,
                        &mut self.s_tx,
                        &mut self.c_tx,
                        state,
                    );

                    // For now, blindly assume the sender has performed
                    // the full burden of verification and that all messages
                    // sent over these IPC channels are valid and should
                    // be sent to anyone subscribed.
                    //
                    // ALSO assume the channel message has the proper name
                    // attached before sending.
                    ctx.send_client(&msg).await.unwrap(); // TODO better error handling
                    state = ctx.into();
                }
                // next_incoming() only returns Err() on fatal errors.
                Err(e) => break Err(e),
            }
        };

        if let Err(_e) = e {
            // TODO: emit session error log?
        }

        // TODO better error cleanup.
        self.r_rx
            .into_inner()
            .unsplit(self.r_tx)
            .shutdown()
            .await
            .unwrap();
        // The other comms primitives are in-process only, not external, so just
        // dropping them is fine.
    }

    /// Waits for the next incoming signal from timeout, client, channel, or server.
    /// Returns the first signal that arrives. Buffers are reused for efficiency.
    ///
    /// The client timeout is only re-set on a message from the remote client. This
    /// timeout is DIFFERENT from [IrcSession::ping_deadline]. This keeps track of
    /// the last time we heard from the client, while `ping_deadline` tracks
    /// PING/PONG response pairs.
    async fn next_incoming<'a>(
        &mut self,
        own_buf: &'a mut Vec<u8>,
        ref_buf: &'a mut Arc<Bytes>,
    ) -> IrcResult<Signal<'a>> {
        select! {
            _ = &mut self.timeout.as_mut() => Ok(Signal::Timeout),
            msg = Self::next_client_msg(&mut self.r_rx, own_buf, b'\n', 10240) => Ok(Signal::Client(msg?)),
            res = Self::next_channel_msg(&mut self.c_rx, ref_buf) => {
                // Ordering here is important - first the response is destructured `?`
                // so that any errors will cause the timer to NOT reset - bad/invalid messages
                // from the remote client won't refresh their grace period.
                let (name, msg) = res?;

                self.timeout.as_mut().reset(Instant::now() + Duration::from_secs(30));

                Ok(Signal::Channel(name, msg))
            },
            msg = Self::next_server_msg(&mut self.s_rx) => Ok(Signal::Server(msg?)),
        }
    }

    async fn next_client_msg<'a, R>(
        reader: &mut R,
        own_buf: &'a mut Vec<u8>,
        delimiter: u8,
        limit: usize,
    ) -> IrcResult<Message<'a>>
    where
        R: AsyncBufRead + Unpin,
    {
        let start_len = own_buf.len();
        reader
            .take(limit as u64)
            .read_until(delimiter, own_buf)
            .await?;

        let bytes_read = own_buf.len() - start_len;

        // If we read the maximum allowed bytes but didn't find the delimiter, error
        if bytes_read >= limit && own_buf.last() != Some(&delimiter) {
            return Err(IrcSessionError::MessageTooLong);
        }

        Ok(ircv3_parse::parse(str::from_utf8(own_buf)?)?)
    }

    async fn next_channel_msg<'a>(
        channels: &mut StreamMap<ChannelName, ChannelSource>,
        ref_buf: &'a mut Arc<Bytes>,
    ) -> IrcResult<(Arc<str>, Message<'a>)> {
        let Some((name, msgbuf)) = channels.next().await else {
            return Err(IrcSessionError::ChannelEOF);
        };

        let _ = std::mem::replace(ref_buf, msgbuf?);

        let msg = ircv3_parse::parse(str::from_utf8(ref_buf)?)?;

        Ok((Arc::clone(&name), msg))
    }

    async fn next_server_msg(reader: &mut ServerSource) -> IrcResult<ServerMessage> {
        Ok(reader.recv().await?)
    }
}

enum Signal<'a> {
    Timeout,
    Server(ServerMessage),
    Client(Message<'a>),
    Channel(ChannelName, Message<'a>),
}
