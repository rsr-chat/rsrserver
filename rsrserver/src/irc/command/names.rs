use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Names;

impl_command_handler!(Names: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Names: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Names: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
