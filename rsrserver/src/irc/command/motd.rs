use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    ext::StrExt,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
};

pub struct Motd;

impl_command_handler!(Motd: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Motd: state::Registered, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let modes = ""; // Supported user and channel modes.
    ctx.send(&format!(":* 422 {nick} rsr-{VERSION} * :{modes}\r\n"))
        .await?;

    // TODO: RPL_ISUPPORT?

    Ok(ctx)
});

impl_command_handler!(Motd: state::Authenticated, async fn handle(ctx, msg) {
    let nick = ctx.nick();
    let nick = nick.slice_at_most(40);

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let modes = ""; // Supported user and channel modes.
    ctx.send(&format!(":* 422 {nick} rsr-{VERSION} * :{modes}\r\n"))
        .await?;

    // TODO: RPL_ISUPPORT?

    Ok(ctx)
});
