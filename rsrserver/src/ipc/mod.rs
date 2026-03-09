use crate::{
    error::{IrcResult, IrcSessionError},
    storage::model::{ChannelId, GuildId},
};

pub mod nats;
mod packet_generated;
use bytes::Bytes;
pub use packet_generated::*;

/// Pub/Sub handler, usually to a backend NATS server
/// but also handles communication internal to the
/// process.
///
/// Keep in mind that the IPC handler only carries
/// notifications, not authoritative data. The DBs
/// remain the authoritative source of truth, and
/// this handler is only meant to inform other
/// nodes that something happened in real-time.
pub trait IpcHandler {
    fn subscribe(&self, topic: &str) -> impl Future<Output = IrcResult<()>> + Sync + Send;

    fn send<'a>(&self, packet: Packet<'a>) -> impl Future<Output = IrcResult<()>> + Sync + Send;

    fn recv<'a>(
        &self,
        subject: &str,
        buffer: &'a mut Bytes,
    ) -> impl Future<Output = IrcResult<Packet<'a>>> + Sync + Send;
}

pub enum Signal<'a> {
    Timeout(crate::storage::model::Snowflake),
    ClientMessage(ircv3_parse::Message<'a>),
    ServerMessage(GuildId, Option<ChannelId>, ircv3_parse::Message<'a>),
    Unknown(Option<&'a str>, Option<&'a [u8]>),
}

impl<'a> TryFrom<packet_generated::Message<'a>> for Signal<'a> {
    type Error = IrcSessionError;
    fn try_from(value: packet_generated::Message<'a>) -> Result<Self, Self::Error> {
        let msg = ircv3_parse::parse(str::from_utf8(value.data().bytes())?)?;
        let guild = GuildId(u64::from_le_bytes(value.guild_id().0).into());
        let channel: Option<ChannelId> = value
            .channel_id()
            .map(|s| ChannelId(u64::from_le_bytes(s.0).into()));
        Ok(Signal::ServerMessage(guild, channel, msg))
    }
}

impl<'a> TryFrom<packet_generated::Timeout<'a>> for Signal<'a> {
    type Error = IrcSessionError;
    fn try_from(value: packet_generated::Timeout) -> Result<Self, Self::Error> {
        Ok(Signal::Timeout(u64::from_le_bytes(value.id().0).into()))
    }
}

impl<'a> TryFrom<packet_generated::Unknown<'a>> for Signal<'a> {
    type Error = IrcSessionError;
    fn try_from(value: packet_generated::Unknown<'a>) -> Result<Self, Self::Error> {
        Ok(Signal::Unknown(
            packet_generated::Signal::Unknown.variant_name(),
            value.data().map(|d| d.bytes())
        ))
    }
}

impl<'a> TryFrom<packet_generated::Packet<'a>> for Signal<'a> {
    type Error = IrcSessionError;
    fn try_from(value: packet_generated::Packet<'a>) -> Result<Self, Self::Error> {
        match value.signal_type() {
            packet_generated::Signal::Message => value
                .signal_as_message()
                .map(|m| m.try_into())
                .unwrap_or_else(|| Ok(
                    Signal::Unknown(
                        packet_generated::Signal::Message.variant_name(),
                        Some(value._tab.buf())
                    ))),
            packet_generated::Signal::Timeout => value
                .signal_as_timeout()
                .map(|m| m.try_into())
                .unwrap_or_else(|| Ok(
                    Signal::Unknown(
                        packet_generated::Signal::Timeout.variant_name(),
                        Some(value._tab.buf())
                    ))),
            x => Ok(Signal::Unknown(x.variant_name(), Some(value._tab.buf())))
        }
    }
}
