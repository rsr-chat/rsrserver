use ircv3_parse::Message;

use crate::{error::IrcResult, irc::{IrcContext, command::CommandHandler, state}, storage::Storage};

pub struct User;

impl CommandHandler<state::Anonymous> for User {
    type Contract = state::Anonymous;

    async fn handle<'a, S: Storage>(
        mut ctx: IrcContext<'a, state::Anonymous, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        ctx.registration_required().await?;
        Ok(ctx)
    }
}

impl CommandHandler<state::Registered> for User {
    type Contract = state::Registered;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Registered, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}

impl CommandHandler<state::Authenticated> for User {
    type Contract = state::Authenticated;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, state::Authenticated, S>,
        _msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>> {
        todo!();
        Ok(ctx)
    }
}
/*
impl CommandHandler<User> for state::Anonymous {
    type Contract = MaybeTransition<state::Anonymous, State>;
    async fn handle<'a, S: Storage>(
        mut ctx: crate::irc::IrcContext<'a, Self, S>,
        msg: ircv3_parse::Message<'a>,
    ) -> crate::error::IrcResult<crate::irc::IrcContext<'a, Self, S>> {
        let client = ctx.client_nick();

        let Some(new_nick) = msg.params().middles.first() else {
            ctx.send_client_unchecked(&format!(":* 431 {client} :No nickname given")).await?;
            return Ok(ctx);
        };

        // TODO: Actual length limits.
        let new_nick = new_nick.slice_at_most(128);

        if let Err(reason) = ctx.validate_nick(new_nick) {
            ctx.send_client_unchecked(&format!(":* 432 {client} {new_nick} :{reason}")).await?;
            return Ok(ctx);
        }

        let Ok(_) = ctx.session().whois(new_nick).await else {
            ctx.send_client_unchecked(&format!(":* 432 {client} {new_nick} :Nickname is already in use")).await?;
            return Ok(ctx);
        };

        ctx.nick = Some(new_nick.to_owned());

        // TODO: Trigger state change.

        Ok(ctx)
    }
}
*/