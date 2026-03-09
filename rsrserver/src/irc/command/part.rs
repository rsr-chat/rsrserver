use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Part;

impl_command_handler!(Part: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Part: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Part: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
