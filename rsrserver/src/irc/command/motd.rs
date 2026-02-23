use ircv3_parse::Message;

use crate::{
    error::IrcResult, ext::StrExt, irc::{GenericStateExt, IrcContext, command::CommandHandler, state}, storage::Storage
};

pub struct Motd;

impl CommandHandler<state::Anonymous> for Motd {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        ctx.registration_required();
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Motd {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);

        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let modes = ""; // Supported user and channel modes.
        ctx.send_client_unchecked(&format!(":* 422 {nick} rsr-{VERSION} * :{modes}\r\n"))
            .await?;

        // TODO: RPL_ISUPPORT?

        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Motd {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);

        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let modes = ""; // Supported user and channel modes.
        ctx.send_client_unchecked(&format!(":* 422 {nick} rsr-{VERSION} * :{modes}\r\n"))
            .await?;

        // TODO: RPL_ISUPPORT?

        Ok(ctx)
    }
}
