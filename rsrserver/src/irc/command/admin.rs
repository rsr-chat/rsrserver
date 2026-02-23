use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    ext::StrExt,
    irc::{GenericStateExt, IrcContext, command::CommandHandler, state},
    storage::Storage,
};

pub struct Admin;

impl CommandHandler<state::Anonymous> for Admin {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        ctx.registration_required().await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Admin {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Admin {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        Self::handle_inner(&mut ctx).await?;
        Ok(ctx)
    }
}

impl Admin {
    async fn handle_inner<'a, T: GenericStateExt, S: Storage>(ctx: &mut IrcContext<'a, T, S>) -> IrcResult<()> {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40).to_owned();

        ctx.send_client_unchecked(&format!(":* 256 {nick} :%INFOHEADER%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 257 {nick} :%ADMINLOC%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 258 {nick} :%ADMINHOST%\r\n"))
            .await?;
        ctx.send_client_unchecked(&format!(":* 257 {nick} :%ADMINEMAIL%\r\n"))
            .await?;

        Ok(())
    }
}

