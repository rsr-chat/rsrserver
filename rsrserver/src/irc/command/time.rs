use chrono::Utc;
use ircv3_parse::Message;

use crate::{error::IrcResult, ext::StrExt, irc::{GenericStateExt, IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Time;

impl CommandHandler<state::Anonymous> for Time {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Time {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Time {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx, &msg).await?;
        Ok(ctx)
    }
}

impl Time {
    pub fn current() -> (i64, String) {
        let now = Utc::now();
        (now.timestamp(), now.to_rfc3339())
    }

    async fn handle_inner<'a, T: GenericStateExt, S: Storage>(ctx: &mut IrcContext<'a, T, S>, msg: &Message<'a>) -> IrcResult<()> {
        let (unix_time, time_str) = Self::current();
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);

        ctx.send_client_unchecked(&format!(":* {nick} * {unix_time} 0 :{time_str}\r\n"))
            .await?;
        Ok(())
    }
}
