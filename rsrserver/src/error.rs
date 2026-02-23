use thiserror::Error;
use tokio::sync::broadcast::error::RecvError;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;


pub type IrcResult<T, E = IrcSessionError> = Result<T, E>;

#[derive(Debug, Error)]
pub enum IrcSessionError {
    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("ParseInt Error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("MessageParse Error: {0}")]
    IRCError(#[from] ircv3_parse::IRCError),

    #[error("Invalid UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    #[error("Channel Stream Recv Error: {0}")]
    ChannelRecvError(#[from] BroadcastStreamRecvError),

    #[error("Server Stream Recv Error: {0}")]
    ServerRecvError(#[from] RecvError),

    #[error("Message Too Long")]
    MessageTooLong,

    #[error("Connection timed out.")]
    Timeout,

    #[error("Channel Map unexpectedly closed")]
    ChannelEOF,

    #[error("Unsupported CAP Version")]
    UnsupportedCap,

    #[error("Client issued QUIT command. Reason: {0}")]
    ClientQUIT(String),
}

pub enum StorageError<E> {
    
    Backend(E),
}