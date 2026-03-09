use std::{
    ops::{Deref, DerefMut},
    time::Duration,
};

use crate::{
    error::{IrcResult, IrcSessionError},
    ext::StrExt,
    ipc::IpcHandler,
    irc::{IntoMessage, IrcSession, state},
    router::Router,
};
use ircv3_parse::{Message, message::ser::ToMessage};
use tokio::{io::{AsyncWrite, AsyncBufRead}, time::Instant};

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
pub struct IrcContext<'a, T, S, R> {
    typestate: T,
    storage: &'a S,
    router: &'a R,
    session: &'a mut IrcSession,
}

impl<'a, T, S, R> IrcContext<'a, T, S, R> {
    pub fn new(state: T, storage: &'a S, router: &'a R, session: &'a mut IrcSession) -> Self {
        Self {
            typestate: state,
            storage,
            router,
            session,
        }
    }
}

impl<'a, T, S, R> IrcContext<'a, T, S, R> {
    /// Transition the internal state object from some state `T`
    /// to another state `U`. This method makes no assumptions
    /// about what `T` and `U` must be.
    ///
    /// Typically, an unconditional state change will have `U`
    /// be some concrete type, while a conditional state change
    /// will have `U` be some [crate::irc::session::TypeState<T_OLD, T_NEW>].
    pub fn transition<U>(self, new: U) -> IrcContext<'a, U, S, R> {
        IrcContext {
            typestate: new,
            storage: self.storage,
            router: self.router,
            session: self.session,
        }
    }
}

impl<T, S, R> IrcContext<'_, T, S, R> {
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
                //self.send_client_unchecked("PING ").await?;
                //self.send_client_unchecked(nonce.to_string()).await?;
                //self.send_client_unchecked("\r\n").await?;

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

impl<'a, T, S, Rt, Rx, Tx> IrcContext<'a, T, S, Router<Rt, Rx, Tx>>
where
    Rt: IpcHandler,
    Rx: AsyncBufRead,
    Tx: AsyncWrite,
    T: GenericStateExt,
{
    pub async fn unknown_command(&mut self, cmd: &str) -> IrcResult<()> {
        let nick = self.typestate.nick();
        let nick = nick.slice_at_most(40);
        let cmd = cmd.slice_at_most(512 - 70);

        let msg = format!(":* 421 {nick} {cmd} :Unknown command\r\n");
        //self.r_tx.write_all(msg.as_ref()).await?;
        //self.r_tx.flush().await?;
        Ok(())
    }

    pub async fn registration_required(&mut self) -> IrcResult<()> {
        let nick = self.typestate.nick();
        let nick = nick.slice_at_most(40);

        let msg = format!(":* 451 {nick} :Registration is required\r\n");
        //self.r_tx.write_all(msg.as_ref()).await?;
        //self.r_tx.flush().await?;
        Ok(())
    }

    pub async fn send<M>(&mut self,  msg: &M) -> IrcResult<()> where M: IntoMessage {
        let msg = msg.into_message()?;

        Ok(())
    }
}

impl<T, S, R> Deref for IrcContext<'_, T, S, R> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.typestate
    }
}

impl<T, S, R> DerefMut for IrcContext<'_, T, S, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.typestate
    }
}

impl<T, S, R> From<IrcContext<'_, T, S, R>> for state::New<T> {
    fn from(value: IrcContext<'_, T, S, R>) -> Self {
        state::New(value.typestate)
    }
}

impl<T, S, R> From<IrcContext<'_, T, S, R>> for state::Old<T> {
    fn from(value: IrcContext<'_, T, S, R>) -> Self {
        state::Old(value.typestate)
    }
}

impl<S, R> From<IrcContext<'_, state::Anonymous, S, R>> for state::Anonymous {
    fn from(value: IrcContext<'_, state::Anonymous, S, R>) -> Self {
        value.typestate
    }
}

impl<S, R> From<IrcContext<'_, state::Registered, S, R>> for state::Registered {
    fn from(value: IrcContext<'_, state::Registered, S, R>) -> Self {
        value.typestate
    }
}

impl<S, R> From<IrcContext<'_, state::Authenticated, S, R>> for state::Authenticated {
    fn from(value: IrcContext<'_, state::Authenticated, S, R>) -> Self {
        value.typestate
    }
}
