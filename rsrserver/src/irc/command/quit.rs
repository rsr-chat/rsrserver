use ircv3_parse::Message;

use crate::{error::{IrcResult, IrcSessionError}, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Quit;

impl CommandHandler<state::Anonymous> for Quit {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        // QUIT on an Anonymous connection is easy: Clients
        // must be registered to join channels or chat, so
        // they can simply be disconnected without any extra
        // handling needed at this point since there's
        // nobody to notify.

        Err::<state::Anonymous, IrcSessionError>(crate::error::IrcSessionError::ClientQUIT(msg.params().trailing.as_str().to_owned()))
    }
}

impl CommandHandler<state::Registered> for Quit {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Quit {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}
