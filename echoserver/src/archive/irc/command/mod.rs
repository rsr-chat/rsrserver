use crate::irc::{IrcContext, IrcResult, IrcState, Anonymous, Registered, Authenticated};
use ircv3_parse::Message;

pub trait Handler<T> {
    async fn handle<'a>(_ctx: IrcContext<'a, T>, _msg: &Message<'_>) -> IrcResult<IrcState>;
}

macro_rules! irc_cmds {
    [$($names:ident),+] => {
        $(
            mod $names;
            pub use $names::*;
        )+

        impl<'a> IrcContext<'a, Anonymous> {
            pub async fn execute<'b>(mut self, msg: &'b Message<'a>) -> IrcResult<IrcState> {
                match msg.command().as_str().to_lowercase().as_str() {
                    $(
                        stringify!($names) => pastey::paste! {
                            crate::irc::command::[<$names:upper_camel>]::handle(self, msg).await
                        },
                    )+
                    cmd => {
                        self.send_unchecked(format!(":* 421 :Unknown command {cmd}\r\n").as_bytes()).await?;
                        Ok(self.into())
                    },
                }
            }   
        }

        impl<'a> IrcContext<'a, Registered> {
            pub async fn execute<'b>(mut self, msg: &'b Message<'a>) -> IrcResult<IrcState> {
                match msg.command().as_str().to_lowercase().as_str() {
                    $(
                        stringify!($names) => pastey::paste! {
                            crate::irc::command::[<$names:upper_camel>]::handle(self, msg).await
                        },
                    )+
                    cmd => {
                        self.send_unchecked(format!(":* 421 :Unknown command {cmd}\r\n").as_bytes()).await?;
                        Ok(self.into())
                    },
                }
            }   
        }

        impl<'a> IrcContext<'a, Authenticated> {
            pub async fn execute<'b>(mut self, msg: &'b Message<'a>) -> IrcResult<IrcState> {
                match msg.command().as_str().to_lowercase().as_str() {
                    $(
                        stringify!($names) => pastey::paste! {
                            crate::irc::command::[<$names:upper_camel>]::handle(self, msg).await
                        },
                    )+
                    cmd => {
                        self.send_unchecked(format!(":* 421 :Unknown command {cmd}\r\n").as_bytes()).await?;
                        Ok(self.into())
                    },
                }
            }   
        }
    };
}

irc_cmds![
    cap,
    authenticate,
    pass,
    nick,
    user,
    ping,
    pong,
    oper,
    quit,
    error,
    join,
    part,
    topic,
    names,
    list,
    invite,
    kick,
    motd,
    version,
    admin,
    connect,
    lusers,
    time,
    stats,
    help,
    info,
    mode,
    privmsg,
    notice,
    who,
    whois,
    whowas,
    kill,
    rehash,
    restart,
    squit,
    away,
    links,
    userhost,
    wallops
];
