pub use super::machine::*;
use tokio::time::Instant;

/// Typestate representing a connection that has not
/// yet registered to the core IRC server.
#[derive(Debug, Default)]
pub struct Anonymous {
    pub nick: Option<String>,
    pub user: Option<String>,
    pub real: Option<String>,
}
impl StateInto<Registered> for Anonymous {}

/// Typestate representing a connection that has
/// registered as a valid IRC client to the core
/// IRC server.
#[derive(Debug)]
pub struct Registered {
    pub nick: String,
    pub user: String,
    pub real: String,

    pub away: Option<String>,
}
impl StateInto<Authenticated> for Registered {}

/// Typestate representing a connection that has
/// fully authenticated itself via the RSR auth
/// capability through its PDS and recieved a
/// proper auth grant from it.
#[derive(Debug)]
pub struct Authenticated {
    pub nick: String,
    pub user: String,
    pub real: String,
    pub expires: Instant,

    pub away: Option<String>,
}
impl StateInto<Registered> for Authenticated {}
