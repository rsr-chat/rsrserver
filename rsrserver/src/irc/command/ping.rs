use ircv3_parse::Message;
use tokio::io::{AsyncBufRead, AsyncWrite};

use crate::{
    error::IrcResult,
    ipc::IpcHandler,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
    router::Router,
    storage::Storage,
};

pub struct Ping;

impl_command_handler!(Ping: state::Anonymous, async fn handle(ctx, msg) {
    Self::pong(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Ping: state::Registered, async fn handle(ctx, msg) {
    Self::pong(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Ping: state::Authenticated, async fn handle(ctx, msg) {
    Self::pong(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl Ping {
    async fn pong<'a, T, S, Rt, Rx, Tx>(
        ctx: &mut IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
        msg: &Message<'a>,
    ) -> IrcResult<()>
    where
        T: GenericStateExt,
        S: Storage,
        Rt: IpcHandler,
        Rx: AsyncBufRead,
        Tx: AsyncWrite,
    {
        let token = msg.params().middles.first().unwrap_or("");
        ctx.send(&format!("PONG {token}\r\n")).await?;

        Ok(())
    }
}
