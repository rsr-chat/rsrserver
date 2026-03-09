use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Whowas;

impl_command_handler!(Whowas: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Whowas: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Whowas: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
