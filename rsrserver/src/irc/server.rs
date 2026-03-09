use std::{net::SocketAddr, pin::Pin, sync::Arc};

use bytes::Bytes;
use tokio::io::{BufReader, split};
use tokio_rustls::server::TlsStream;

use crate::{
    error::{IrcResult, IrcSessionError},
    ipc::{IpcHandler, Signal},
    irc::{
        IrcContext, IrcSession, command,
        state::{self, MaybeTransition, Old},
    },
    router::Router,
    storage::Storage,
    tls::{ClientSink, ClientSource, TlsHandler},
};

pub struct IrcServer<S, Rt> {
    realtime: Arc<Rt>,
    storage: Arc<S>,
}

impl<S, Rt> IrcServer<S, Rt> {
    pub fn new(realtime: Arc<Rt>, storage: Arc<S>) -> Self {
        Self {
            realtime,
            storage,
        }
    }
}

impl<S, Rt> TlsHandler for IrcServer<S, Rt>
where
    S: Storage + Send + Sync + 'static,
    Rt: IpcHandler + Unpin + Send + Sync + 'static,
{
    type Future = Pin<Box<dyn Future<Output = ()> + Send + Sync>>;

    fn handle(
        &mut self,
        stream: TlsStream<tokio::net::TcpStream>,
        client_addr: SocketAddr,
    ) -> Self::Future {
        let (rx, tx) = split(stream);

        let storage = Arc::clone(&self.storage);
        let router = Router::new(Arc::clone(&self.realtime), BufReader::new(rx), tx);
        Box::pin(
            IrcConnection {
                storage,
                router,
                client_addr,
            }
            .start(),
        )
    }
}

pub struct IrcConnection<S, Rt> {
    // Storage backend
    storage: Arc<S>,

    // Real-time message Rx/Tx
    router: Router<Rt, ClientSource, ClientSink>,

    // Remote host address
    client_addr: SocketAddr,
}

impl<S, Rt> IrcConnection<S, Rt>
where
    S: Storage,
    Rt: IpcHandler + Unpin,
{
    pub async fn start(mut self) {
        match self.run().await {
            Ok(()) => panic!("IrcConnection::run() returned OK!"),
            Err(e) => self.die_nice(e).await,
        }
    }

    async fn run(&mut self) -> IrcResult<()> {
        // Stores data that must be read in from our owned-handle (i.e. Client rx)
        let mut own_buf = Vec::<u8>::new();

        // Stores data shared over pipe
        let mut ref_buf = Bytes::new();

        // Stores data shared over NATS
        let mut ipc_buf = Bytes::new();

        let mut session = IrcSession::new();

        // Helper macro to quickly create a context given a state variable.
        macro_rules! context {
            ($state:ident) => {
                IrcContext::new($state, self.storage.as_ref(), &self.router, &mut session)
            };
        }

        // Helper macro to quickly define a state machine loop that
        // only exits on a state /change/ or an error.
        macro_rules! state_machine {
            ($state:ident) => {
                loop {
                    let signal = self
                        .router
                        .next(&mut own_buf, &mut ref_buf, &mut ipc_buf)
                        .await?;

                    let res = match signal {
                        Signal::Timeout(id) => {
                            let mut ctx = context!($state);
                            ctx.ping_keepalive().await.unwrap(); // TODO better error handling
                            Ok(Old(ctx).into())
                        }
                        Signal::ClientMessage(msg) => command::route(context!($state), msg).await,
                        Signal::ServerMessage(guild, channel, msg) => todo!(),
                        Signal::Unknown(_, _) => continue,
                    };

                    match res? {
                        MaybeTransition::Old(o) => $state = o,
                        MaybeTransition::New(n) => break n,
                    }
                }
            };
        }

        let mut anon = state::Anonymous::default();
        let mut reg: state::Registered = state_machine!(anon);
        loop {
            let mut auth: state::Authenticated = state_machine!(reg);
            reg = state_machine!(auth);
        }
    }

    async fn die_nice(self, _e: IrcSessionError) -> () {
        // TODO handle graceful connection shutdown
    }
}
