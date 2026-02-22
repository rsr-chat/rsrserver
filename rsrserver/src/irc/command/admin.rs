use crate::irc::{command::CommandHandler, state};

pub struct Admin;

impl CommandHandler<Admin> for state::Anonymous {
    async fn handle<'a>(
        mut ctx: crate::irc::IrcContext<'a, Self>,
        _msg: ircv3_parse::Message<'a>,
    ) -> crate::error::IrcResult<crate::irc::IrcContext<'a, Self>> {
        let nick = ctx.nick().to_owned();

        ctx.send_client_unchecked(&format!(":* 256 {nick} :%INFOHEADER%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 257 {nick} :%ADMINLOC%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 258 {nick} :%ADMINHOST%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 257 {nick} :%ADMINEMAIL%\r\n"))
            .await?;

        Ok(ctx)
    }
}
