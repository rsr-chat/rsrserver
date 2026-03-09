use std::sync::Arc;
use bitflags::bitflags;

macro_rules! container_impl {
    ($to:path, $ti:path) => {
        impl From<$ti> for $to {
            fn from(value: $ti) -> Self {
                Self(value)
            }
        }

        impl From<$to> for $ti {
            fn from(value: $to) -> Self {
                value.0
            }
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Snowflake(pub u64);
container_impl!(Snowflake, u64);

lazy_static::lazy_static! {
    static ref DEFAULT_GUILD_ID: Did = Did(Arc::from(""));
}
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Did(pub Arc<str>);
container_impl!(Did, Arc<str>);
impl Default for Did {
    fn default() -> Self {
        Did(Arc::clone(&DEFAULT_GUILD_ID.0))
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Hostmask(pub Arc<str>);
container_impl!(Hostmask, Arc<str>);


#[derive(Eq, PartialEq, Hash, Debug)]
pub struct InviteId(pub Snowflake);
container_impl!(InviteId, Snowflake);

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct BanId(pub Snowflake);
container_impl!(BanId, Snowflake);

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct UserId(pub Snowflake);
container_impl!(UserId, Snowflake);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GuildId(pub Snowflake);
container_impl!(GuildId, Snowflake);
impl Default for GuildId {
    fn default() -> Self {
        Self(Snowflake(0))
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ChannelId(pub Snowflake);
container_impl!(ChannelId, Snowflake);

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct RoleId(pub Snowflake);
container_impl!(RoleId, Snowflake);

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct MessageId(pub Snowflake);
container_impl!(MessageId, Snowflake);

pub struct Message {
    
}

pub struct Role {

}

pub struct Channel {

}

pub struct Guild {
    /// The guild's DID, in a format that can be found
    /// via simple lookup. This yields the PDS endpoint.
    did: Did,

    /// Scoped access key given to the server by the
    /// guild's PDS allowing the server to access data
    /// on behalf of the guild without changing critical
    /// guild configuration.
    access_key: (),

    /// Signing key given to the server on behalf of
    /// the guild Owner allowing the server to sign
    /// signals as having actually gone through the
    /// guild's designated router.
    guild_signing_key: (),

    /// Key used to encrypt and decrypt private records
    /// that live on the PDS but shouldn't be publicly
    /// viewable.
    crypto_key: (),
}

pub struct User {

}

pub struct Ban {

}

pub struct Invite {

}

bitflags! {
    pub struct PermissionSet: u64 {

    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::empty()
    }
}

pub enum ServerAuditEvent {

}

pub enum GuildAuditEvent {

}

pub enum MessageAuditEvent {

}

pub enum UserAuditEvent {

}