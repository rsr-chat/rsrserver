use ircv3_parse::Message;
use tokio::time::Instant;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Pong;

impl CommandHandler<state::Anonymous> for Pong {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Pong {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Pong {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl Pong {
    async fn handle_inner<'a, T, S: Storage>(ctx: &mut IrcContext<'a, T, S>, msg: &Message<'a>) -> IrcResult<()> {
        // Are we waiting on a PONG?
        let Some((deadline, expected_token)) = ctx.session_mut().ping_deadline() else {
            // Drop PONG messages when we're not waiting for any.
            return Ok(());
        };

        // Has the deadline passed?
        if Instant::now() > *deadline {
            return Ok(());
        }

        // Did the client return the correct token?
        let Ok(client_token) = msg.params().middles.first().unwrap_or("").parse::<u64>() else {
            // Drop malformed PONG replies.
            return Ok(());
        };

        if *expected_token == client_token {
            // Client sent back an appropriate PONG within
            // the time limit - clear the waiting ping.
            *ctx.session_mut().ping_deadline() = None;
        }

        Ok(())
    }
}
