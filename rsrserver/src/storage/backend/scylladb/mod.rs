use std::{borrow::Borrow, net::SocketAddr, time::Duration};
use scylla::{client::{session::Session, session_builder::SessionBuilder}, errors::{DbError, NewSessionError}};
use thiserror::Error;

use crate::storage::{backend::{StorageResult, StoreInit, TimeSeriesStore}, model::*};

#[derive(Debug, Error)]
pub enum ScyllaInitError {
    #[error("Error connecting to ScyllaDB cluster: {0}")]
    Connect(#[from] NewSessionError),

    #[error("Error migrating ScyllaDB from version {from} to {to}: {inner}")]
    Migrate {
        from: u64,
        to: u64,
        inner: DbError,
    },
}

pub struct ScyllaDB {
    session: Session
}

impl StoreInit for ScyllaDB {
    async fn connect<T>(seeds: impl IntoIterator<Item = T>) -> StorageResult<Self> where T: Into<SocketAddr>, Self: Sized {
        todo!()
    }
}

impl ScyllaDB {
    pub async fn connect(nodes: impl IntoIterator<Item = impl Borrow<SocketAddr>>) -> Result<Self, ScyllaInitError> {
        let session: Session = SessionBuilder::new()
        .connection_timeout(Duration::from_secs(3))
        .cluster_metadata_refresh_interval(Duration::from_secs(10))
        .compression(None)
        .known_nodes_addr(nodes)
        .build()
        .await?;

        let mut s = Self {
            session
        };

        s.migrate().await?;

        Ok(s)
    }

    async fn migrate(&mut self) -> Result<(), ScyllaInitError> {
        // Read current version out of DB, or else default to 0.
        let version = 0;

        
        Ok(())
    }
}

impl TimeSeriesStore for ScyllaDB {
    async fn get_messages_latest(
        &self,
        n_messages: u8,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(MessageId, Message)>> {
        todo!()
    }
    
    async fn get_messages_around(
        &self,
        id: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
        range_before: u8,
        range_after: u8,
    ) -> StorageResult<Vec<(MessageId, Message)>> {
        todo!()
    }
    
    async fn get_message_by_id(
        &self,
        id: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Message>> {
        todo!()
    }
    
    async fn add_message(
        &self,
        msg: impl Into<Message>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<MessageId> {
        todo!()
    }
    
    async fn del_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Message>> {
        todo!()
    }
    
    async fn set_pinned_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()> {
        todo!()
    }
    
    async fn del_pinned_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()> {
        todo!()
    }
    
    async fn get_pinned_messages(
        &self,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(MessageId, Message)>> {
        todo!()
    }
    
    async fn get_server_audit_log(
        &self,
    ) -> StorageResult<Vec<ServerAuditEvent>> {
        todo!()
    }
    
    async fn add_server_audit_log(
        &self,
        event: impl Into<ServerAuditEvent>,
    ) -> StorageResult<()> {
        todo!()
    }
    
    async fn get_guild_audit_log(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<GuildAuditEvent>> {
        todo!()
    }
    
    async fn add_guild_audit_log(
        &self,
        event: impl Into<GuildAuditEvent>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()> {
        todo!()
    }
    
    async fn get_message_audit_log(
        &self,
        message: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<MessageAuditEvent>> {
        todo!()
    }
    
    async fn add_message_audit_log(
        &self,
        event: impl Into<MessageAuditEvent>,
        message: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()> {
        todo!()
    }
}
