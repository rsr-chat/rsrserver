use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Pass;

impl_command_handler!(Pass: state::Anonymous, async fn handle(ctx, msg) {
    // Silently allow any PASS commands as RSR servers do not
    // support PASS authentication.

    Ok(ctx)
});

impl_command_handler!(Pass: state::Registered, async fn handle(ctx, msg) {
    // Silently allow any PASS commands as RSR servers do not
    // support PASS authentication.

    Ok(ctx)
});

impl_command_handler!(Pass: state::Authenticated, async fn handle(ctx, msg) {
    // Silently allow any PASS commands as RSR servers do not
    // support PASS authentication.

    Ok(ctx)
});
