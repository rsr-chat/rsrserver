use crate::{
    error::IrcResult,
    irc::{
        GenericStateExt, IrcContext,
        state::{MaybeTransition, Old, StateInto},
    },
    storage::Storage,
};
use ircv3_parse::Message;

pub trait CommandHandler<T> {
    type Contract;

    async fn handle<'a, S: Storage>(
        ctx: IrcContext<'a, T, S>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>>;
}

fn _route<'a, T, U, S, C>(
    ctx: IrcContext<'a, T, S>,
    msg: Message<'a>,
) -> impl Future<Output = IrcResult<impl Into<C::Contract>>>
where
    C: CommandHandler<T>,
    C::Contract: Into<MaybeTransition<T, U>>,
    S: Storage,
    T: StateInto<U>,
{
    C::handle(ctx, msg)
}

macro_rules! commands {
    [$($cmd:ident),+] => {
        $(
            pastey::paste! {
                mod [<$cmd:lower>];
                pub use [<$cmd:lower>]::$cmd;
            }
        )+

        pub async fn route<'a, T, U, S: Storage>(
            mut ctx: IrcContext<'a, T, S>,
            msg: Message<'a>
        ) -> IrcResult<MaybeTransition<T, U>>
        where
            // All routeable commands must:
            // - Implement IRC command handlers.
            // - Be able to transform into some specified next state.
            // - Be extractable from the Context Object.
            // - Support all Contextual Operations.
            // - Return some type that converts into a transition object.
            T: StateInto<U> + From<IrcContext<'a, T, S>> + GenericStateExt + 'a,
            $($cmd : CommandHandler<T>,)+
            $(<$cmd as CommandHandler<T>>::Contract: Into<MaybeTransition<T, U>>,)+
        {
            pastey::paste! {
                match msg.command().as_str() {
                    $(
                        stringify!([<$cmd:upper>]) => {
                            let c = _route
                            ::<T, U, S, $cmd>(ctx, msg)
                            .await?;

                            let c: <$cmd as CommandHandler<T>>::Contract = c.into();

                            Ok(Into::<MaybeTransition<T, U>>::into(c))
                        }
                    )+
                    cmd => {
                        ctx.unknown_command(cmd).await?;
                        Ok(Old(ctx).into())
                    },
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
