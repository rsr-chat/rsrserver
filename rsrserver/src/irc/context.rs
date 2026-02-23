use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use ircv3_parse::Message;
use tokio::{io::AsyncWriteExt, time::Instant};
use tokio_stream::StreamMap;

use crate::{
    error::{IrcResult, IrcSessionError},
    ext::StrExt,
    irc::{ChannelName, ChannelSink, ClientSink, IrcSession, ServerSink, state},
    storage::Storage,
};

/// An ephemeral object that borrows all possible state
/// relevant to a single request, and takes ownership of
/// the current typestate. When dropped, all borrows are
/// returned, and the typestate is lost. In order to retain
/// the typestate, it must be extracted via `Into`.
///
/// Most commonly, this struct is converted in-situ by an
/// IRC handler as it's internal type is converted into
/// [TypeState] to handle potential state changes using
/// [IrcContext::]
pub struct IrcContext<'a, T, S> {
    storage: &'a S,
    session: &'a mut IrcSession,

    /// Stream back to Client
    r_tx: &'a mut ClientSink,
    /// Stream back to Server
    s_tx: &'a mut ServerSink,
    /// Streams back to Channels
    c_tx: &'a mut StreamMap<ChannelName, ChannelSink>,

    typestate: T,
}

impl<'a, T, S> IrcContext<'a, T, S> {
    pub fn new(
        storage: &'a S,
        session: &'a mut IrcSession,
        r_tx: &'a mut ClientSink,
        s_tx: &'a mut ServerSink,
        c_tx: &'a mut StreamMap<ChannelName, ChannelSink>,
        state: T,
    ) -> Self {
        Self {
            storage,
            session,
            r_tx,
            s_tx,
            c_tx,
            typestate: state,
        }
    }
}

impl<'a, T, S> IrcContext<'a, T, S> {
    /// Transition the internal state object from some state `T`
    /// to another state `U`. This method makes no assumptions
    /// about what `T` and `U` must be.
    ///
    /// Typically, an unconditional state change will have `U`
    /// be some concrete type, while a conditional state change
    /// will have `U` be some [crate::irc::session::TypeState<T_OLD, T_NEW>].
    pub fn transition<U>(self, new: U) -> IrcContext<'a, U, S> {
        IrcContext {
            storage: self.storage,
            session: self.session,
            r_tx: self.r_tx,
            s_tx: self.s_tx,
            c_tx: self.c_tx,
            typestate: new,
        }
    }
}

impl<T, S> IrcContext<'_, T, S>
{    
    pub fn apply(self) -> T {
        self.typestate
    }

    pub fn session(&self) -> &IrcSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut IrcSession {
        &mut self.session
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

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
            }
            None => {
                // No awaiting ping, so send one out.
                let deadline = Instant::now() + Duration::from_secs(8);
                let nonce: u64 = rand::random();
                self.send_client_unchecked("PING ").await?;
                self.send_client_unchecked(nonce.to_string()).await?;
                self.send_client_unchecked("\r\n").await?;

                *self.session.ping_deadline() = Some((deadline, nonce));

                Ok(())
            }
        }
    }

    pub fn validate_nick(&self, _nick: &str) -> Result<(), &str> {
        // TODO: Nick validaton rules.
        Ok(())
    }
}

/// Helper trait that allows for helper methods on the outer context object that
/// internally depend on typestate internals despite not exposing them in their
/// function signatures/requiring them as explicit parameters.
pub trait GenericStateExt {
    fn nick(&self) -> &str;
    fn user(&self) -> &str;
    fn real(&self) -> &str;
    fn away(&self) -> Option<&str>;
}

impl GenericStateExt for state::Anonymous {
    fn nick(&self) -> &str {
        self.nick.as_deref().unwrap_or("*")
    }

    fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("*")
    }

    fn real(&self) -> &str {
        self.real.as_deref().unwrap_or("")
    }

    fn away(&self) -> Option<&str> {
        None
    }
}

impl GenericStateExt for state::Registered {
    fn nick(&self) -> &str {
        &self.nick
    }

    fn user(&self) -> &str {
        &self.user
    }

    fn real(&self) -> &str {
        &self.real
    }

    fn away(&self) -> Option<&str> {
        self.away.as_deref()
    }
}

impl GenericStateExt for state::Authenticated {
    fn nick(&self) -> &str {
        &self.nick
    }

    fn user(&self) -> &str {
        &self.user
    }

    fn real(&self) -> &str {
        &self.real
    }

    fn away(&self) -> Option<&str> {
        self.away.as_deref()
    }
}

impl<'a, T, S> IrcContext<'a, T, S>
where
    S: Storage,
    T: GenericStateExt
{
    pub async fn unknown_command(&mut self, cmd: &str) -> IrcResult<()> {
        let nick = self.typestate.nick();
        let nick = nick.slice_at_most(40);
        let cmd = cmd.slice_at_most(512 - 70);

        let msg = format!(":* 421 {nick} {cmd} :Unknown command\r\n");
        self.r_tx.write_all(msg.as_ref()).await?;
        self.r_tx.flush().await?;
        Ok(())
    }

    pub async fn registration_required(&mut self) -> IrcResult<()> {
        let nick = self.typestate.nick();
        let nick = nick.slice_at_most(40);
        
        let msg = format!(":* 451 {nick} :Registration is required\r\n");
        self.r_tx.write_all(msg.as_ref()).await?;
        self.r_tx.flush().await?;
        Ok(())
    }
}

impl<T, S> Deref for IrcContext<'_, T, S> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.typestate
    }
}

impl<T, S> DerefMut for IrcContext<'_, T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.typestate
    }
}

impl<T, S> From<IrcContext<'_, T, S>> for state::New<T> {
    fn from(value: IrcContext<'_, T, S>) -> Self {
        state::New(value.typestate)
    }
}

impl<T, S> From<IrcContext<'_, T, S>> for state::Old<T> {
    fn from(value: IrcContext<'_, T, S>) -> Self {
        state::Old(value.typestate)
    }
}

impl<S> From<IrcContext<'_, state::Anonymous, S>> for state::Anonymous {
    fn from(value: IrcContext<'_, state::Anonymous, S>) -> Self {
        value.typestate
    }
}

impl<S> From<IrcContext<'_, state::Registered, S>> for state::Registered {
    fn from(value: IrcContext<'_, state::Registered, S>) -> Self {
        value.typestate
    }
}

impl<S> From<IrcContext<'_, state::Authenticated, S>> for state::Authenticated {
    fn from(value: IrcContext<'_, state::Authenticated, S>) -> Self {
        value.typestate
    }
}
