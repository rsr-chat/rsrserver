use chrono::Utc;
use ircv3_parse::Message;
use tokio::io::{AsyncBufRead, AsyncWrite};

use crate::{
    error::IrcResult,
    ext::StrExt,
    ipc::IpcHandler,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
    router::Router,
    storage::Storage,
};

pub struct Time;

impl_command_handler!(Time: state::Anonymous, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Time: state::Registered, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Time: state::Authenticated, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl Time {
    pub fn current() -> (i64, String) {
        let now = Utc::now();
        (now.timestamp(), now.to_rfc3339())
    }

    async fn handle_inner<'a, T, S, Rt, Rx, Tx>(
        ctx: &mut IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
        _msg: &Message<'a>,
    ) -> IrcResult<()>
    where
        T: GenericStateExt,
        S: Storage,
        Rt: IpcHandler,
        Rx: AsyncBufRead,
        Tx: AsyncWrite,
    {
        let (unix_time, time_str) = Self::current();
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);

        ctx.send(&format!(":* {nick} * {unix_time} 0 :{time_str}\r\n"))
            .await?;
        Ok(())
    }
}
