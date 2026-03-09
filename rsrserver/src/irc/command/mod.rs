use crate::{
    error::IrcResult,
    ipc::IpcHandler,
    irc::{
        GenericStateExt, IrcContext,
        state::{MaybeTransition, Old, StateInto},
    },
    router::Router,
    storage::Storage,
};
use ircv3_parse::Message;

#[macro_export]
macro_rules! impl_command_handler {
    (
        $cmd:ty:
        $state:ty,
        async fn handle($ctx:ident, $msg:ident) $body:block
    ) => {
        impl_command_handler!{$cmd: $state >> $state, async fn handle($ctx, $msg) $body}
    };

    (
        $cmd:ty:
        $state:ty >> $contract:ty,
        async fn handle($ctx:ident, $msg:ident) $body:block
    ) => {
        impl CommandHandler<$state> for $cmd {
            type Contract = $contract;

            #[allow(unused_mut, unused)]
            async fn handle<'a, S, Rt, Rx, Tx>(
                mut $ctx: IrcContext<'a, $state, S, crate::router::Router<Rt, Rx, Tx>>,
                $msg: Message<'a>,
            ) -> IrcResult<impl Into<Self::Contract>>
            where
                S: crate::storage::Storage,
                Rt: crate::ipc::IpcHandler,
                Rx: tokio::io::AsyncBufRead,
                Tx: tokio::io::AsyncWrite,
            $body
        }
    };
}

pub trait CommandHandler<T> {
    type Contract;

    async fn handle<'a, S, Rt, Rx, Tx>(
        ctx: IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
        msg: Message<'a>,
    ) -> IrcResult<impl Into<Self::Contract>>
    where
        S: Storage,
        Rt: IpcHandler,
        Rx: tokio::io::AsyncBufRead,
        Tx: tokio::io::AsyncWrite;
}

fn _route<'a, T, U, S, C, Rt, Rx, Tx>(
    ctx: IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
    msg: Message<'a>,
) -> impl Future<Output = IrcResult<impl Into<C::Contract>>>
where
    C: CommandHandler<T>,
    C::Contract: Into<MaybeTransition<T, U>>,
    S: Storage,
    T: StateInto<U>,
    Rt: IpcHandler,
    Rx: tokio::io::AsyncBufRead,
    Tx: tokio::io::AsyncWrite,
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

        pub async fn route<'a, T, U, S, Rt, Rx, Tx>(
            mut ctx: IrcContext<'a, T, S, Router<Rt, Rx, Tx>>,
            msg: Message<'a>
        ) -> IrcResult<MaybeTransition<T, U>>
        where
            // All routeable commands must:
            // - Implement IRC command handlers.
            // - Be able to transform into some specified next state.
            // - Be extractable from the Context Object.
            // - Support all Contextual Operations.
            // - Return some type that converts into a transition object.
            T: StateInto<U> + From<IrcContext<'a, T, S, Router<Rt, Rx, Tx>>> + GenericStateExt + 'a,
            S: Storage,
            Rt: IpcHandler,
            Rx: tokio::io::AsyncBufRead,
            Tx: tokio::io::AsyncWrite,
            $($cmd : CommandHandler<T>,)+
            $(<$cmd as CommandHandler<T>>::Contract: Into<MaybeTransition<T, U>>,)+
        {
            pastey::paste! {
                match msg.command().as_str() {
                    $(
                        stringify!([<$cmd:upper>]) => {
                            let c = _route
                            ::<T, U, S, $cmd, Rt, Rx, Tx>(ctx, msg)
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
