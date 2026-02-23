use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Ping;

impl CommandHandler<state::Anonymous> for Ping {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::pong(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Ping {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::pong(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Ping {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::pong(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl Ping {
    async fn pong<'a, T, S: Storage>(ctx: &mut IrcContext<'a, T, S>, msg: &Message<'a>) -> IrcResult<()> {
        let token = msg.params().middles.first().unwrap_or("");
        ctx.send_client_unchecked(&format!("PONG {token}\r\n")).await?;

        Ok(())
    }
}