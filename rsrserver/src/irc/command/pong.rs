use ircv3_parse::Message;
use tokio::{io::{AsyncBufRead, AsyncWrite}, time::Instant};

use crate::{
    error::IrcResult, ipc::IpcHandler, irc::{GenericStateExt, IrcContext, command::CommandHandler, state}, router::Router, storage::Storage
};

pub struct Pong;

impl_command_handler!(Pong: state::Anonymous, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Pong: state::Registered, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl_command_handler!(Pong: state::Authenticated, async fn handle(ctx, msg) {
    Self::handle_inner(&mut ctx, &msg).await?;
    Ok(ctx)
});

impl Pong {
    async fn handle_inner<'a, T, S, Rt, Rx, Tx>(
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
