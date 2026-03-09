use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    ext::StrExt,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
};

pub struct Help;
impl_command_handler!(Help: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Help: state::Registered, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);
    ctx.send(&format!(":* 524 {nick} * :Not yet implemented\r\n")).await?;
    Ok(ctx)
});

impl_command_handler!(Help: state::Authenticated, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);
    ctx.send(&format!(":* 524 {nick} * :Not yet implemented\r\n")).await?;
    Ok(ctx)
});
