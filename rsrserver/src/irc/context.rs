use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use ircv3_parse::Message;
use tokio::{
    io::AsyncWriteExt,
    time::Instant,
};
use tokio_stream::StreamMap;

use crate::{
    error::{IrcResult, IrcSessionError},
    irc::{ChannelName, ChannelSink, ClientSink, IrcSession, ServerSink, state::{self, Anonymous}},
};

pub struct IrcContext<'a, T> {
    session: &'a mut IrcSession,

    /// Stream back to Client
    r_tx: &'a mut ClientSink,
    /// Stream back to Server
    s_tx: &'a mut ServerSink,
    /// Streams back to Channels
    c_tx: &'a mut StreamMap<ChannelName, ChannelSink>,

    state: T,
}

impl<'a, T> IrcContext<'a, T> {
    pub fn new(
        session: &'a mut IrcSession,
        r_tx: &'a mut ClientSink,
        s_tx: &'a mut ServerSink,
        c_tx: &'a mut StreamMap<ChannelName, ChannelSink>,
        state: T,
    ) -> Self {
        Self {
            session,
            r_tx,
            s_tx,
            c_tx,
            state,
        }
    }
}

impl<T> Deref for IrcContext<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T> DerefMut for IrcContext<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl From<IrcContext<'_, Anonymous>> for Anonymous {
    fn from(value: IrcContext<'_, Anonymous>) -> Self {
        value.state
    }
}

impl<T> IrcContext<'_, T> {
    pub async fn send_client_unchecked<'a>(&'a mut self, msg: impl AsRef<[u8]>) -> IrcResult<()> {
        self.r_tx.write_all(msg.as_ref()).await?;
        self.r_tx.flush().await?;
        Ok(())
    }

    pub async fn send_client<'a>(&'a mut self, msg: &'a Message<'a>) -> IrcResult<()> {
        self.send_client_unchecked(msg.input_raw()).await
    }

    pub async fn ping_keepalive(&mut self) -> IrcResult<()> {
        match self.session.ping_deadline() {
            Some((deadline, _)) => {
                if Instant::now() > *deadline {
                    // Timer expired!
                    Err(IrcSessionError::Timeout)
                } else {
                    // Timer not yet expired, keep waiting.
                    Ok(())
                }
            },
            None => {
                // No awaiting ping, so send one out.
                let deadline = Instant::now() + Duration::from_secs(8);
                let nonce: u64 = rand::random();
                self.send_client_unchecked("PING ").await?;
                self.send_client_unchecked(nonce.to_string()).await?;
                self.send_client_unchecked("\r\n").await?;

                *self.session.ping_deadline() = Some((deadline, nonce));

                Ok(())
            },
        }
    }

    pub async fn unknown_command(&mut self, cmd: &str) -> IrcResult<()> {
        todo!();
    }
}

impl<'a> IrcContext<'a, state::Anonymous> {
    pub fn nick(&'a self) -> &'a str {
        self.state.nick.as_deref().unwrap_or("*")
    }
}
