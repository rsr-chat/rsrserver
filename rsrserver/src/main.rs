use std::error::Error as StdError;
use std::net::IpAddr;
use std::sync::Arc;
use std::{net::ToSocketAddrs, ops::Deref};

use async_nats::ServerAddr;
use color_eyre::eyre::Result;

use crate::cfg::{CliArgs, FileConfig, ServeCommand};
use crate::ipc::nats::NatsClient;
use crate::irc::IrcServer;
use crate::storage::backend::cockroachdb::CockroachDB;
use crate::storage::backend::scylladb::ScyllaDB;
use crate::storage::datastore::DataStore;
use crate::tls::{TlsServer, TlsServerConfig};

mod cfg;
mod error;
mod ext;
mod ipc;
mod irc;
mod router;
mod storage;
mod tls;

lazy_static::lazy_static! {
    static ref ARGS: CliArgs = argh::from_env();
}

lazy_static::lazy_static! {
    static ref CONFIG: FileConfig = {
        let path = &ARGS.config_path;
        if !path.exists() {
            panic!("Provided config file path does not exist.");
        }

        toml::from_str(
            &std::fs::read_to_string(&path)
            .expect("Error reading config file.")
        ).expect("Error parsing config file.")
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + 'static>> {
    println!("{:#?}", &ARGS.deref());

    match &ARGS.subcommand {
        cfg::SubCommand::Serve(c) => serve(c).await,
        cfg::SubCommand::Status(_) => todo!(),
        cfg::SubCommand::Doctor(_) => todo!(),
    }
}

async fn serve(_: &ServeCommand) -> Result<(), Box<dyn StdError + 'static>> {
    let addr = CONFIG
        .bind
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))?;

    let t_seeds: Result<Vec<(IpAddr, u16)>, _> =
        CONFIG.deref().db.scylla_seeds().into_iter().collect();
    let g_seeds: Result<Vec<(IpAddr, u16)>, _> =
        CONFIG.deref().db.cockroach_seeds().into_iter().collect();
    let storage = Arc::new(DataStore::<ScyllaDB, CockroachDB>::new(t_seeds?, g_seeds?).await?);

    let tls_cfg = TlsServerConfig {
        addr,
        cert: CONFIG.tls_cert.clone(),
        key: CONFIG.tls_key.clone(),
    };

    let r_seeds: Result<Vec<ServerAddr>, _> = CONFIG.deref().db.nats_seeds().into_iter().collect();
    let r_options = async_nats::ConnectOptions::new();
    let realtime = Arc::new(NatsClient::connect(r_seeds?, r_options).await?);

    TlsServer::create(tls_cfg)
        .await?
        .serve(IrcServer::new(realtime, storage))
        .await;

    Ok(())
}
