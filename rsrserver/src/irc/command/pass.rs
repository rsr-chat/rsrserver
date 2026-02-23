use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Pass;

impl CommandHandler<state::Anonymous> for Pass {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Silently allow any PASS commands as RSR servers do not 
        // support PASS authentication.

        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Pass {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Silently allow any PASS commands as RSR servers do not 
        // support PASS authentication.

        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Pass {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // Silently allow any PASS commands as RSR servers do not 
        // support PASS authentication.

        Ok(ctx)
    }
}
