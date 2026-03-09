use async_nats::{ConnectError, ConnectOptions, ToServerAddrs};

use crate::ipc::IpcHandler;

pub struct NatsClient {
    /// Handle to the underlying remote NATS server.
    client: async_nats::Client,
}

unsafe impl Send for NatsClient {}
unsafe impl Sync for NatsClient {}

impl NatsClient {
    pub async fn connect<A: ToServerAddrs>(addrs: A, options: ConnectOptions) -> Result<Self, ConnectError>  {
        Ok(Self {
            client: async_nats::connect_with_options(addrs, options).await?
        })
    }
}

impl IpcHandler for NatsClient {
    async fn subscribe(&self, topic: &str) -> crate::error::IrcResult<()> {
        todo!()
    }

    async fn send<'a>(&self, packet: super::Packet<'a>) -> crate::error::IrcResult<()> {
        todo!()
    }

    async fn recv<'a>(
        &self,
        subject: &str,
        buffer: &'a mut bytes::Bytes,
    ) -> crate::error::IrcResult<super::Packet<'a>> {
        todo!()
    }
}