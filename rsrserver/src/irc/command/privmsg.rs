use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Privmsg;

impl CommandHandler<state::Anonymous> for Privmsg {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        ctx.registration_required();
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Privmsg {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Privmsg {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}