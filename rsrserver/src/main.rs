use std::io;
use std::net::ToSocketAddrs;
use std::path::PathBuf;

use std::error::Error as StdError;

use argh::FromArgs;
use color_eyre::eyre::Result;

use crate::{irc::IrcServer, tls::{TlsServer, TlsServerConfig}};

mod ext;
mod error;
mod storage;
mod tls;
mod irc;

/// Tokio Rustls server example
#[derive(FromArgs)]
struct Options {
    /// bind addr
    #[argh(positional)]
    addr: String,

    /// cert file
    #[argh(option, short = 'c')]
    cert: PathBuf,

    /// key file
    #[argh(option, short = 'k')]
    key: PathBuf,
}

lazy_static::lazy_static! {
    static ref OPTIONS: Options = argh::from_env();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError + Send + Sync + 'static>> {
    let addr = OPTIONS
        .addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::AddrNotAvailable))?;

    TlsServer::create(TlsServerConfig {
        addr,
        cert: OPTIONS.cert.clone(),
        key: OPTIONS.key.clone(),
    })
    .await?
    .serve(IrcServer::new(()))
    .await;

    Ok(())
}
