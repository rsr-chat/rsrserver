use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Connect;

impl_command_handler!(Connect: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Connect: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Connect: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
