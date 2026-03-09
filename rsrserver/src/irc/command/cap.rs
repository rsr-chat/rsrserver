use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}};

pub struct Cap;

impl_command_handler!(Cap: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Cap: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Cap: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
