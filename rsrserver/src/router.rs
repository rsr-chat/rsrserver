use std::{pin::Pin, sync::Arc, time::Duration};

use bytes::Bytes;
use dashmap::{DashMap, mapref::one::Ref};
use tokio::{
    io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt, AsyncWrite},
    select,
    time::{Instant, Sleep, sleep},
};
use tokio_stream::{StreamExt, StreamMap};

use crate::{
    error::{IrcResult, IrcSessionError},
    ipc::{IpcHandler, Signal},
    storage::model::{ChannelId, GuildId, Snowflake},
};

pub type ChannelSource = tokio_stream::wrappers::BroadcastStream<Bytes>;
pub type ChannelSink = tokio::sync::broadcast::WeakSender<Bytes>;

pub type ServerSource = StreamMap<GuildId, StreamMap<ChannelId, ChannelSource>>;
pub type ServerSink = StreamMap<GuildId, StreamMap<ChannelId, ChannelSink>>;

pub struct Router<Rt, Rx, Tx> {
    realtime: Arc<Rt>,
    client_rx: Rx,
    client_tx: Tx,
    server_rx: ServerSource,
    server_tx: ServerSink,
    timeout: Pin<Box<Sleep>>,
}

impl<Rt, Rx, Tx> Router<Rt, Rx, Tx> {
    pub fn new(realtime: Arc<Rt>, client_rx: Rx, client_tx: Tx) -> Self {
        Self {
            realtime,
            client_rx,
            client_tx,
            server_rx: StreamMap::new(),
            server_tx: StreamMap::new(),
            timeout: Box::pin(sleep(Duration::from_secs(15))),
        }
    }
}

impl<Rt, Rx, Tx> Router<Rt, Rx, Tx>
where
    Rt: IpcHandler + Unpin,
    Rx: AsyncBufRead + Unpin,
    Tx: AsyncWrite + Unpin,
{
    pub async fn next<'a>(
        &mut self,
        client_buf: &'a mut Vec<u8>,
        chan_buf: &'a mut Bytes,
        ipc_buf: &'a mut Bytes,
    ) -> Result<Signal<'a>, IrcSessionError> {
        select! {
            _ = &mut self.timeout.as_mut() => Ok(Signal::Timeout(Snowflake(0))),
            cl_s = Self::next_local_client_msg(&mut self.client_rx, client_buf, b'\n', 10240) => cl_s,
            ch_s = Self::next_local_channel_msg(&mut self.server_rx, chan_buf) => {
                // Ordering here is important - first the response is destructured `?`
                // so that any errors will cause the timer to NOT reset - bad/invalid messages
                // from the remote client won't refresh their grace period.
                let ch_s = ch_s?;

                self.timeout.as_mut().reset(Instant::now() + Duration::from_secs(30));

                Ok(ch_s)
            },
            rt_s = Self::next_rt_ipc_msg(&self.realtime, ipc_buf) => rt_s,
        }
    }

    async fn next_local_client_msg<'a>(
        reader: &mut Rx,
        client_buf: &'a mut Vec<u8>,
        delimiter: u8,
        limit: usize,
    ) -> IrcResult<Signal<'a>>
    where
        Rx: Unpin,
    {
        let start_len = client_buf.len();
        reader
            .take(limit as u64)
            .read_until(delimiter, client_buf)
            .await?;

        let bytes_read = client_buf.len() - start_len;

        // If we read the maximum allowed bytes but didn't find the delimiter, error
        if bytes_read >= limit && client_buf.last() != Some(&delimiter) {
            return Err(IrcSessionError::MessageTooLong);
        }

        Ok(Signal::ClientMessage(ircv3_parse::parse(str::from_utf8(
            client_buf,
        )?)?))
    }

    async fn next_local_channel_msg<'a>(
        guilds: &mut ServerSource,
        chan_buf: &'a mut Bytes,
    ) -> IrcResult<Signal<'a>> {
        let Some((guild, (chan, bytes))) = guilds.next().await else {
            return Err(IrcSessionError::ChannelEOF);
        };

        let _ = std::mem::replace(chan_buf, bytes?);

        let msg = ircv3_parse::parse(str::from_utf8(chan_buf)?)?;

        Ok(Signal::ServerMessage(
            guild.clone(),
            Some(chan.clone()),
            msg,
        ))
    }

    async fn next_rt_ipc_msg<'a>(rt_ipc: &Rt, ipc_buf: &'a mut Bytes) -> IrcResult<Signal<'a>>
    where
        Rt: Unpin,
    {
        match rt_ipc.recv("*", ipc_buf).await {
            Ok(pckt) => Ok(pckt.try_into()?),
            Err(e) => Err(e.into()),
        }
    }
}

lazy_static::lazy_static! {
    static ref REGISTRY: GuildManager = GuildManager::default();
}

pub struct GuildManager {
    guilds: DashMap<GuildId, ChannelManager>,
}

impl GuildManager {
    pub async fn get<'a>(&'a self, guild: &GuildId) -> Option<Ref<'a, GuildId, ChannelManager>> {
        self.guilds.get(guild)
    }
}

impl Default for GuildManager {
    fn default() -> Self {
        let guilds = DashMap::new();
        guilds.insert(GuildId::default(), ChannelManager::new(GuildId::default()));
        Self { guilds }
    }
}

pub struct ChannelManager {
    guild_id: GuildId,
    channels: DashMap<ChannelId, (ChannelSource, ChannelSink)>,
}

impl ChannelManager {
    fn new(guild_id: GuildId) -> Self {
        Self {
            guild_id,
            channels: DashMap::new(),
        }
    }
}
