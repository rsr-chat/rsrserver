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

pub struct Admin;

impl_command_handler!(Admin: state::Anonymous,  async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Admin: state::Registered, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx).await?;
    Ok(ctx)
});

impl_command_handler!(Admin: state::Authenticated, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx).await?;
    Ok(ctx)
});

impl Admin {
    async fn handle_inner<'a, T, S, Rt, Rx, Tx>(
        ctx: &mut IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
    ) -> IrcResult<()>
    where
        T: GenericStateExt,
        S: Storage,
        Rt: IpcHandler,
        Rx: AsyncBufRead,
        Tx: AsyncWrite,
    {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40).to_owned();

        ctx.send(&format!(":* 256 {nick} :%INFOHEADER%\r\n")).await?;
        ctx.send(&format!(":* 257 {nick} :%ADMINLOC%\r\n")).await?;
        ctx.send(&format!(":* 258 {nick} :%ADMINHOST%\r\n")).await?;
        ctx.send(&format!(":* 257 {nick} :%ADMINEMAIL%\r\n")).await?;

        Ok(())
    }
}
