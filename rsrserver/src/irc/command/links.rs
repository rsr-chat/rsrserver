use ircv3_parse::Message;

use crate::{error::IrcResult, ext::StrExt, irc::{GenericStateExt, IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct Links;

impl CommandHandler<state::Anonymous> for Links {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        ctx.registration_required();
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for Links {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);
        ctx.send_client_unchecked(&format!(":* 365 {nick}  :End of /LINKS list\r\n")).await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for Links {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        let nick = ctx.nick();
        let nick = nick.slice_at_most(40);
        ctx.send_client_unchecked(&format!(":* 365 {nick}  :End of INFO list\r\n")).await?;
        Ok(ctx)
    }
}