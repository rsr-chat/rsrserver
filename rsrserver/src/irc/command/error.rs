use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Error;

impl CommandHandler<state::Anonymous> for Error {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Clients should never send this message.
        // Be nice and just ignore it.
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Error {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Clients should never send this message.
        // Be nice and just ignore it.
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Error {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Clients should never send this message.
        // Be nice and just ignore it.
        Ok(ctx)
    }
}