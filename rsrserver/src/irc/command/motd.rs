use crate::irc::{command::CommandHandler, state};

pub struct Motd;

impl CommandHandler<Motd> for state::Anonymous {
    async fn handle<'a>(
        mut ctx: crate::irc::IrcContext<'a, Self>,
        _msg: ircv3_parse::Message<'a>,
    ) -> crate::error::IrcResult<crate::irc::IrcContext<'a, Self>> {
        let nick = ctx.nick().to_owned();
        
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let modes = ""; // Supported user and channel modes.
        ctx.send_client_unchecked(&format!(":* 422 {nick} rsr-{VERSION} * :{modes}\r\n"))
            .await?;

        // TODO: RPL_ISUPPORT?

        Ok(ctx)
    }
}
