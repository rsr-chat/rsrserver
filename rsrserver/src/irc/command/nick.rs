use ircv3_parse::Message;

use crate::{
    error::IrcResult,
    irc::{IrcContext, command::CommandHandler, state},
};

pub struct Nick;

impl_command_handler!(Nick: state::Anonymous, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Nick: state::Registered, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

impl_command_handler!(Nick: state::Authenticated, async fn handle(ctx, msg) {
    ctx.registration_required().await?;
    Ok(ctx)
});

/*
      let client = ctx.client_nick();

       let Some(new_nick) = msg.params().middles.first() else {
           ctx.send_client_unchecked(&format!(":* 431 {client} :No nickname given")).await?;
           return Ok(Old(ctx).into());
       };

       // TODO: Actual length limits.
       let new_nick = new_nick.slice_at_most(128);

       if let Err(reason) = ctx.validate_nick(new_nick) {
           ctx.send_client_unchecked(&format!(":* 432 {client} {new_nick} :{reason}")).await?;
           return Ok(Old(ctx).into());
       }

       let Ok(_) = ctx.storage().whois(new_nick).await else {
           ctx.send_client_unchecked(&format!(":* 432 {client} {new_nick} :Nickname is already in use")).await?;
           return Ok(Old(ctx).into());
       };

       ctx.nick = Some(new_nick.to_owned());

       // TODO: Trigger try state change.
       Ok(Old(ctx).into())
*/
