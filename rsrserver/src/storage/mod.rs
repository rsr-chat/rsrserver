use std::sync::Arc;

use crate::error::StorageError;

mod irc_model;

pub type StorageResult<T, E> = Result<T, StorageError<E>>;

pub trait Storage: Send + Sync {
    type Error;

    /// Retrieve WHOIS information for the given nick, or None if
    /// the Nick is not registered.
    async fn whois(&self, nick: &str) -> StorageResult<Option<irc_model::Whois>, Self::Error>;
}

// () is a dummy provider that no-ops everything.
#[cfg(debug_assertions)]
impl Storage for () {
    type Error = ();

    async fn whois(&self, _nick: &str) -> StorageResult<Option<irc_model::Whois>, Self::Error> {
        todo!()
    }
}

impl<T> Storage for Arc<T> where T: Storage {
    type Error = T::Error;
    async fn whois(&self, nick: &str) -> StorageResult<Option<irc_model::Whois>, Self::Error> {
        self.as_ref().whois(nick).await
    }
}