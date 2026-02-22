use chrono::Utc;

use crate::irc::{command::CommandHandler, state};

pub struct Time;

impl Time {
    pub fn current() -> (i64, String) {
        let now = Utc::now();
        (now.timestamp(), now.to_rfc3339())
    }
}

impl CommandHandler<Time> for state::Anonymous {
    async fn handle<'a>(
        mut ctx: crate::irc::IrcContext<'a, Self>,
        _msg: ircv3_parse::Message<'a>,
    ) -> crate::error::IrcResult<crate::irc::IrcContext<'a, Self>> {
        let (unix_time, time_str) = Time::current();
        let nick = ctx.nick();
        ctx.send_client_unchecked(&format!(":* {nick} * {unix_time} 0 :{time_str}\r\n"))
            .await?;

        Ok(ctx)
    }
}
