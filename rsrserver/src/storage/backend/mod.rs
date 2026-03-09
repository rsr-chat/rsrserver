
pub mod cockroachdb;
pub mod scylladb;

use std::net::SocketAddr;

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    error::StorageError,
    storage::model::{
        Ban, BanId, Channel, ChannelId, Guild, GuildAuditEvent, GuildId, Hostmask, Invite, InviteId, Message, MessageAuditEvent, MessageId, PermissionSet, Role, RoleId, ServerAuditEvent, UserId, User
    },
};

pub type StorageResult<T, E=StorageError> = Result<T, E>;

pub trait StoreInit {
    async fn connect<T>(seeds: impl IntoIterator<Item = T>) -> StorageResult<Self> where T: Into<SocketAddr>, Self: Sized;
}

/// Time-series message data for messages and audit logs, usually Cassandra or Scylla.
pub trait TimeSeriesStore: Send + Sync {
    /// Fetch the N latest messages in a given channel.
    async fn get_messages_latest(
        &self,
        n_messages: u8,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(MessageId, Message)>>;

    /// Fetch the given range of messages around the given channel.
    /// The message itself is included in the query result.
    async fn get_messages_around(
        &self,
        id: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
        range_before: u8,
        range_after: u8,
    ) -> StorageResult<Vec<(MessageId, Message)>>;

    /// Fetch a single message by its ID.
    async fn get_message_by_id(
        &self,
        id: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Message>>;

    /// Add a message to the database. If the message already exists, it is soft-updated.
    async fn add_message(
        &self,
        msg: impl Into<Message>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<MessageId>;

    /// Soft-delete a message from the database.
    async fn del_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Message>>;

    /// Add a message to the quick-retrieval list for a channel.
    async fn set_pinned_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;

    /// Remove a message from the quick-retrieval list for a channel.
    async fn del_pinned_message(
        &self,
        msg: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;

    /// Get all messages in the quick-retrieval list for a channel.
    async fn get_pinned_messages(
        &self,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(MessageId, Message)>>;

    async fn get_server_audit_log(
        &self,
    ) -> StorageResult<Vec<ServerAuditEvent>>;

    async fn add_server_audit_log(
        &self,
        event: impl Into<ServerAuditEvent>,
    ) -> StorageResult<()>;

    async fn get_guild_audit_log(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<GuildAuditEvent>>;

    async fn add_guild_audit_log(
        &self,
        event: impl Into<GuildAuditEvent>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;

    async fn get_message_audit_log(
        &self,
        message: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<MessageAuditEvent>>;

    async fn add_message_audit_log(
        &self,
        event: impl Into<MessageAuditEvent>,
        message: impl Into<MessageId>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;
}

/// Relational data for everything else, usually Postgres or Cockroach
pub trait ServerDataStore: Send + Sync {
    /// Retrieve a list of all guilds this server is associated with.
    async fn get_guilds(&self) -> StorageResult<Vec<GuildId>>;
    async fn get_guild(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Guild>>;
    async fn add_guild(&self, guild: Guild) -> StorageResult<GuildId>;
    async fn del_guild(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Guild>>;
    async fn set_guild_suspended(
        &self,
        guild: impl Into<GuildId>,
        suspended: bool,
    ) -> StorageResult<Option<Guild>>;

    async fn get_guild_channels(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(ChannelId, Channel)>>;
    async fn get_channel(
        &self,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Channel>;
    async fn add_channel(
        &self,
        channel: Channel,
        guild: impl Into<GuildId>,
    ) -> StorageResult<ChannelId>;
    async fn del_channel(
        &self,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Channel>;

    /// Reorder channels within a guild. Takes a full ordered list of
    /// channel IDs and updates positions in a single transaction.
    async fn reorder_channels(
        &self,
        guild: impl Into<GuildId>,
        order: &[ChannelId],
    ) -> StorageResult<()>;

    async fn get_roles(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(RoleId, Role)>>;
    async fn get_role_members(
        &self,
        role: impl Into<RoleId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(UserId, User)>>;
    async fn get_role(
        &self,
        role: impl Into<RoleId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Role>;
    async fn add_role(
        &self,
        role: Role,
        guild: impl Into<GuildId>,
    ) -> StorageResult<RoleId>;
    async fn del_role(
        &self,
        role: impl Into<RoleId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Role>;
    /// Reorder roles within a guild. Position determines authority hierarchy.
    async fn reorder_roles(
        &self,
        guild: impl Into<GuildId>,
        order: &[RoleId],
    ) -> StorageResult<()>;

    async fn get_user_roles(
        &self,
        user: impl Into<UserId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(RoleId, Role)>>;
    async fn add_user_role(
        &self,
        user: impl Into<UserId>,
        role: impl Into<RoleId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;
    async fn del_user_role(
        &self,
        user: impl Into<UserId>,
        role: impl Into<RoleId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<()>;

    async fn get_guild_bans(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<Hostmask>>;
    async fn add_guild_ban(
        &self,
        target: impl Into<Hostmask>,
        guild: impl Into<GuildId>,
        ban: Ban,
    ) -> StorageResult<BanId>;
    async fn del_guild_ban(
        &self,
        target: impl Into<Hostmask>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Ban>;

    async fn get_channel_bans(
        &self,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<Hostmask>>;
    async fn add_channel_ban(
        &self,
        target: impl Into<Hostmask>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
        ban: Ban,
    ) -> StorageResult<BanId>;
    async fn del_channel_ban(
        &self,
        target: impl Into<Hostmask>,
        channel: impl Into<ChannelId>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Ban>;
    async fn check_ban(
        &self,
        user: impl Into<UserId>,
        hostmask: &Hostmask,
        channel: Option<impl Into<ChannelId>>,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Option<Ban>>;

    async fn get_guild_invites(
        &self,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Vec<(InviteId, Invite)>>;
    async fn add_guild_invite(
        &self,
        guild: impl Into<GuildId>,
        invite: Invite,
    ) -> StorageResult<InviteId>;
    async fn del_guild_invite(
        &self,
        invite: InviteId,
        guild: impl Into<GuildId>,
    ) -> StorageResult<Invite>;
    async fn redeem_invite_atomic(
        &self,
        code: impl Into<InviteId>,
    ) -> StorageResult<Option<(InviteId, Invite)>>;

    /// Purge all expired entries, general DB cleanup.
    async fn purge(&self) -> StorageResult<()>;

    async fn get_server_config<T: DeserializeOwned>(&self, key: &str) -> StorageResult<T>;
    async fn set_server_config<T: Serialize>(&self, key: &str, val: &T) -> StorageResult<()>;
}

pub trait PermissionResolverExt {
    fn resolve_permissions(&self) -> PermissionSet;
}

impl<T: PermissionResolverExt> PermissionResolverExt for &[T] {
    fn resolve_permissions(&self) -> PermissionSet {
        self.iter().fold(PermissionSet::default(), |acc, r| {
            acc | r.resolve_permissions()  // assuming PermissionSet implements BitOr
        })
    }
}
