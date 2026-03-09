use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    ext::StrExt,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
};

pub struct Info;

impl_command_handler!(Info: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Info: state::Registered, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);
    ctx.send(&format!(":* 374 {nick}  :End of INFO list\r\n"))
        .await?;
    Ok(ctx)
});

impl_command_handler!(Info: state::Authenticated, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);
    ctx.send(&format!(":* 374 {nick}  :End of INFO list\r\n"))
        .await?;
    Ok(ctx)
});
