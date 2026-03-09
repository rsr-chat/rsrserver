use crate::storage::backend::{ServerDataStore, StoreInit};


pub struct CockroachDB {
}

impl StoreInit for CockroachDB {
    async fn connect<T>(seeds: impl IntoIterator<Item=T>) -> super::StorageResult<Self> where T: Into<std::net::SocketAddr>, Self: Sized {
        todo!()
    }
}

impl ServerDataStore for CockroachDB {
    async fn get_guilds(&self) -> super::StorageResult<Vec<crate::storage::model::GuildId>> {
        todo!()
    }

    async fn get_guild(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Option<crate::storage::model::Guild>> {
        todo!()
    }

    async fn add_guild(&self, guild: crate::storage::model::Guild) -> super::StorageResult<crate::storage::model::GuildId> {
        todo!()
    }

    async fn del_guild(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Option<crate::storage::model::Guild>> {
        todo!()
    }

    async fn set_guild_suspended(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
        suspended: bool,
    ) -> super::StorageResult<Option<crate::storage::model::Guild>> {
        todo!()
    }

    async fn get_guild_channels(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<(crate::storage::model::ChannelId, crate::storage::model::Channel)>> {
        todo!()
    }

    async fn get_channel(
        &self,
        channel: impl Into<crate::storage::model::ChannelId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Channel> {
        todo!()
    }

    async fn add_channel(
        &self,
        channel: crate::storage::model::Channel,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::ChannelId> {
        todo!()
    }

    async fn del_channel(
        &self,
        channel: impl Into<crate::storage::model::ChannelId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Channel> {
        todo!()
    }

    async fn reorder_channels(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
        order: &[crate::storage::model::ChannelId],
    ) -> super::StorageResult<()> {
        todo!()
    }

    async fn get_roles(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<(crate::storage::model::RoleId, crate::storage::model::Role)>> {
        todo!()
    }

    async fn get_role_members(
        &self,
        role: impl Into<crate::storage::model::RoleId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<(crate::storage::model::UserId, crate::storage::model::User)>> {
        todo!()
    }

    async fn get_role(
        &self,
        role: impl Into<crate::storage::model::RoleId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Role> {
        todo!()
    }

    async fn add_role(
        &self,
        role: crate::storage::model::Role,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::RoleId> {
        todo!()
    }

    async fn del_role(
        &self,
        role: impl Into<crate::storage::model::RoleId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Role> {
        todo!()
    }

    async fn reorder_roles(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
        order: &[crate::storage::model::RoleId],
    ) -> super::StorageResult<()> {
        todo!()
    }

    async fn get_user_roles(
        &self,
        user: impl Into<crate::storage::model::UserId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<(crate::storage::model::RoleId, crate::storage::model::Role)>> {
        todo!()
    }

    async fn add_user_role(
        &self,
        user: impl Into<crate::storage::model::UserId>,
        role: impl Into<crate::storage::model::RoleId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<()> {
        todo!()
    }

    async fn del_user_role(
        &self,
        user: impl Into<crate::storage::model::UserId>,
        role: impl Into<crate::storage::model::RoleId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<()> {
        todo!()
    }

    async fn get_guild_bans(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<crate::storage::model::Hostmask>> {
        todo!()
    }

    async fn add_guild_ban(
        &self,
        target: impl Into<crate::storage::model::Hostmask>,
        guild: impl Into<crate::storage::model::GuildId>,
        ban: crate::storage::model::Ban,
    ) -> super::StorageResult<crate::storage::model::BanId> {
        todo!()
    }

    async fn del_guild_ban(
        &self,
        target: impl Into<crate::storage::model::Hostmask>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Ban> {
        todo!()
    }

    async fn get_channel_bans(
        &self,
        channel: impl Into<crate::storage::model::ChannelId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<crate::storage::model::Hostmask>> {
        todo!()
    }

    async fn add_channel_ban(
        &self,
        target: impl Into<crate::storage::model::Hostmask>,
        channel: impl Into<crate::storage::model::ChannelId>,
        guild: impl Into<crate::storage::model::GuildId>,
        ban: crate::storage::model::Ban,
    ) -> super::StorageResult<crate::storage::model::BanId> {
        todo!()
    }

    async fn del_channel_ban(
        &self,
        target: impl Into<crate::storage::model::Hostmask>,
        channel: impl Into<crate::storage::model::ChannelId>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Ban> {
        todo!()
    }

    async fn check_ban(
        &self,
        user: impl Into<crate::storage::model::UserId>,
        hostmask: &crate::storage::model::Hostmask,
        channel: Option<impl Into<crate::storage::model::ChannelId>>,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Option<crate::storage::model::Ban>> {
        todo!()
    }

    async fn get_guild_invites(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<Vec<(crate::storage::model::InviteId, crate::storage::model::Invite)>> {
        todo!()
    }

    async fn add_guild_invite(
        &self,
        guild: impl Into<crate::storage::model::GuildId>,
        invite: crate::storage::model::Invite,
    ) -> super::StorageResult<crate::storage::model::InviteId> {
        todo!()
    }

    async fn del_guild_invite(
        &self,
        invite: crate::storage::model::InviteId,
        guild: impl Into<crate::storage::model::GuildId>,
    ) -> super::StorageResult<crate::storage::model::Invite> {
        todo!()
    }

    async fn redeem_invite_atomic(
        &self,
        code: impl Into<crate::storage::model::InviteId>,
    ) -> super::StorageResult<Option<(crate::storage::model::InviteId, crate::storage::model::Invite)>> {
        todo!()
    }

    async fn purge(&self) -> super::StorageResult<()> {
        todo!()
    }

    async fn get_server_config<T: serde::de::DeserializeOwned>(&self, key: &str) -> super::StorageResult<T> {
        todo!()
    }

    async fn set_server_config<T: serde::Serialize>(&self, key: &str, val: &T) -> super::StorageResult<()> {
        todo!()
    }
}