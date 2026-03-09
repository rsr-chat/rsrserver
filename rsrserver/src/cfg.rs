use std::{
    io, net::{AddrParseError, IpAddr, SocketAddr}, path::PathBuf
};

use argh::FromArgs;
use async_nats::ServerAddr;
use serde::{Deserialize, Serialize};

/// Tokio Rustls server example
#[derive(FromArgs, Debug)]
pub struct CliArgs {
    /// configuration TOML file
    #[argh(
        option,
        short = 'c',
        default = "PathBuf::from(\"/etc/rsr/server.toml\")"
    )]
    pub config_path: PathBuf,

    #[argh(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommand {
    Serve(ServeCommand),
    Status(StatusCommand),
    Doctor(DoctorCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "serve")]
pub struct ServeCommand {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "status")]
pub struct StatusCommand {
    #[argh(switch)]
    /// whether to fooey
    fooey: bool,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Third subcommand
#[argh(subcommand, name = "doctor")]
pub struct DoctorCommand {
    #[argh(subcommand)]
    action: Option<DoctorSubCommand>,
}

#[derive(FromArgs, PartialEq, Debug)]
/// rsrserver doctor subcommand.
#[argh(subcommand)]
pub enum DoctorSubCommand {
    DefaultConfig(DefaultConfig),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Empty arguments.
#[argh(subcommand, name = "default-config")]
pub struct DefaultConfig {}

#[derive(Serialize, Deserialize)]
pub struct FileConfig {
    /// Socket to bind the server TLS listener to.
    pub bind: SocketAddr,

    /// TLS Public Key
    pub tls_cert: PathBuf,

    /// TLS Private Key
    pub tls_key: PathBuf,

    /// Database config subfields
    pub db: DbConfig,

    /// ATProto + Extensions config subfields
    pub identity: Identity,
}

#[derive(Serialize, Deserialize)]
pub struct DbConfig {
    /// List of (IP/Hostname, Port) to seed ScyllaDB nodes.
    pub scylla_seeds: Vec<(String, u16)>,

    /// List of (IP/Hostname, Port) to seed CockroachDB nodes.
    pub cockroach_seeds: Vec<(String, u16)>,

    /// List of (IP/Hostname, Port) to seed NATS nodes.
    pub nats_seeds: Vec<String>,
}

impl DbConfig {
    pub fn scylla_seeds(&self) -> impl Iterator<Item = Result<(IpAddr, u16), AddrParseError>> {
        self.scylla_seeds
            .iter()
            .map(|(a, p)| a.parse().map(|a| (a, *p)))
    }

    pub fn cockroach_seeds(&self) -> impl Iterator<Item = Result<(IpAddr, u16), AddrParseError>> {
        self.cockroach_seeds
            .iter()
            .map(|(a, p)| a.parse().map(|a| (a, *p)))
    }

    pub fn nats_seeds(&self) -> impl Iterator<Item = Result<ServerAddr, io::Error>> {
        self.nats_seeds
            .iter()
            .map(|a| a.parse())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Identity {
    /// The private key used to cryptographically fingerprint this
    /// server. If this ever changes, the server is considered a
    /// different server.
    pub signing_key: (),

    /// The public key used to cryptographically fingerprint this
    /// server. If this ever changes, the server is considered a
    /// different server.
    pub signing_cert: (),
}
