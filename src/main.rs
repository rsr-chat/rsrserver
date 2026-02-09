use std::io;
use std::net::ToSocketAddrs;
use std::path::PathBuf;

use std::error::Error as StdError;

use argh::FromArgs;
use color_eyre::eyre::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

use crate::tlsserver::{TlsServer, TlsServerConfig};

mod tlsserver;

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

const WORKER_POOL_SIZE: usize = 128;

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
        pool_size: WORKER_POOL_SIZE,
    })
    .await?
    .serve(&OPTIONS, async move |cfg, stream, addr| {
        let (reader, mut writer) = tokio::io::split(stream);
        let mut reader = tokio::io::BufReader::new(reader);
        let mut line = String::new();

        loop {
            line.clear();

            match reader.read_line(&mut line).await.unwrap() {
                0 => {
                    // Connection closed
                    break;
                }
                _ => {
                    // Echo the line back (including the newline)
                    writer.write_all(line.as_bytes()).await.unwrap();
                    writer.flush().await.unwrap();
                }
            }
        }
    })
    .await;

    Ok(())
}
