use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Error;

impl_command_handler!(Error: state::Anonymous, async fn handle(ctx, msg) {
    // Clients should never send this message.
    // Be nice and just ignore it.
    Ok(ctx)
});

impl_command_handler!(Error: state::Registered, async fn handle(ctx, msg) {
    // Clients should never send this message.
    // Be nice and just ignore it.
    Ok(ctx)
});

impl_command_handler!(Error: state::Authenticated, async fn handle(ctx, msg) {
    // Clients should never send this message.
    // Be nice and just ignore it.
    Ok(ctx)
});
