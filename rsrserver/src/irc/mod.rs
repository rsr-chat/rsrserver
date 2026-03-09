mod server;
use ircv3_parse::{Message, message::ser::ToMessage};
pub use server::*;

mod session;
pub use session::*;

mod context;
pub use context::*;

mod capability;
pub use capability::*;

use crate::error::IrcResult;

pub mod command;

pub trait IntoMessage {
    fn into_message<'a>(&'a self) -> IrcResult<Message<'a>>;
}

impl<T> IntoMessage for T where T: AsRef<str> {
    fn into_message<'a>(&'a self) -> IrcResult<Message<'a>> {
        Ok(ircv3_parse::parse(self.as_ref())?)
    }
}
