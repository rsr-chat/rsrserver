use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Authenticate;

impl_command_handler!(Authenticate: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Authenticate: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Authenticate: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
