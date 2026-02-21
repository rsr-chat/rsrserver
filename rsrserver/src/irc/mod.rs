
mod server;
pub use server::*;

mod session;
pub use session::*;

mod context;
pub use context::*;

mod capability;
pub use capability::*;

pub mod command;

use std::sync::Arc;
use bytes::Bytes;
use tokio::io::BufReader;

type ClientSource = BufReader<tokio::io::ReadHalf<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>>;
type ClientSink = tokio::io::WriteHalf<tokio_rustls::server::TlsStream<tokio::net::TcpStream>>;

type ServerSource = tokio::sync::broadcast::Receiver<ServerMessage>;
type ServerSink = tokio::sync::broadcast::WeakSender<ServerMessage>;

type ChannelSource = tokio_stream::wrappers::BroadcastStream<Arc<Bytes>>;
type ChannelSink = tokio::sync::broadcast::WeakSender<Arc<Bytes>>;

pub type ChannelName = Arc<str>;
pub type ServerMessage = ();
