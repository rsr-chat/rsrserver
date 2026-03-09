use ircv3_parse::Message;

use crate::{
    error::{IrcResult, IrcSessionError},
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Quit;

impl_command_handler!(Quit: state::Anonymous, async fn handle(ctx, msg) {
    // QUIT on an Anonymous connection is easy: Clients
    // must be registered to join channels or chat, so
    // they can simply be disconnected without any extra
    // handling needed at this point since there's
    // nobody to notify.

    Err::<state::Anonymous, IrcSessionError>(crate::error::IrcSessionError::ClientQUIT(msg.params().trailing.as_str().to_owned()))
});

impl_command_handler!(Quit: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Quit: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});
