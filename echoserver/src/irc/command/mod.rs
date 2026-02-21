use crate::{error::IrcResult, irc::IrcContext};
use ircv3_parse::Message;

macro_rules! commands {
    [$($cmd:ident),+] => {
        $(
            pastey::paste! {
                mod [<$cmd:lower>];
                pub use [<$cmd:lower>]::$cmd;
            }
        )+

        pub async fn _route<'a, T>(mut ctx: IrcContext<'a, T>, msg: Message<'a>) -> IrcResult<IrcContext<'a, T>> {
            pastey::paste! {
                match msg.command().as_str() {
                    $(
                        stringify!([<$cmd:upper>]) => todo!(), //$cmd::execute(),
                    )+
                    cmd => ctx.unknown_command(cmd).await.map(|_| ctx),
                }
            }
        }
    };
}

commands![
    Cap,
    Authenticate,
    Pass,
    Nick,
    User,
    Ping,
    Pong,
    Oper,
    Quit,
    Error,
    Join,
    Part,
    Topic,
    Names,
    List,
    Invite,
    Kick,
    Motd,
    Version,
    Admin,
    Connect,
    Lusers,
    Time,
    Stats,
    Help,
    Info,
    Mode,
    Privmsg,
    Notice,
    Who,
    Whois,
    Whowas,
    Kill,
    Rehash,
    Restart,
    Squit,
    Away,
    Links,
    Userhost,
    Wallops
];
