use std::net::SocketAddr;

use crate::storage::{Storage, backend::{ServerDataStore, StorageResult, StoreInit, TimeSeriesStore}};

pub struct DataStore<T, G> {
    /// Time-Series Datastore
    tds: T,

    /// Server DataStore
    sds: G,
}

impl<T, G> DataStore<T, G>
where
    T: StoreInit + TimeSeriesStore,
    G: StoreInit + ServerDataStore,
{
    pub async fn new<I>(t_seeds: impl IntoIterator<Item = I>, g_seeds: impl IntoIterator<Item = I>) -> StorageResult<Self> where I: Into<SocketAddr> {
        Ok(Self {
            tds: T::connect(t_seeds).await?,
            sds: G::connect(g_seeds).await?,
        })
    }
}

impl<T, G> Storage for DataStore<T, G>
where
    T: StoreInit + TimeSeriesStore,
    G: StoreInit + ServerDataStore,
{

}