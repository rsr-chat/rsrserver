/*!

    An IRC session as RSR sees it follows this graph:

            ┌───────────┐                    
            │ Anonymous │                    
            └─────┬─────┘                    
                  │                          
                  │  IRC Standard Registration                
                  │                          
           ┌──────▼──────┐                   
           │ Registered  ◀────┐              
           └──────┬──────┘    │              
                  │           │              
       PDS Grant  │           │ Grant Expires
                  │           │              
           ┌──────▼──────┐    │              
           │Authenticated├────┘              
           └─────────────┘                   

    Client registration lasts the entire duration of
    the connection, but PDS Authentication can and will
    expire, which requires Re-auth via specific commands
    enabled by certain rsr.chat capabilities.

    State changes are performed via typestate pattern:
    all methods which can possibly change state must
    consume the state instance and return an instance
    of [TypeState]. This forces consumers of the state
    instance to handle state changes at compile time.
*/
use tokio::time::Instant;
use crate::irc::Capabilities;

mod machine;
pub mod state;

/// [IrcSession] represents all state-related data that is
/// common to all states.
pub struct IrcSession {
    caps_version: u16,
    caps: Capabilities,

    ping_deadline: Option<(Instant, u64)>,
}

impl IrcSession {
    /// Create a new IrcSession with no capabilities
    /// enabled and a CAP version of 0.
    pub fn new() -> Self {
        Self {
            caps_version: 0,
            caps: Capabilities::empty(),
            ping_deadline: None,
        }
    }

    /// Retrieve the [Capabilities] bitfield
    /// for this session.
    pub fn caps(&self) -> &Capabilities {
        &self.caps
    }

    /// Retrieve the [Capabilities] bitfield
    /// for this session.
    pub fn caps_mut(&mut self) -> &mut Capabilities {
        &mut self.caps
    }

    /// Return the capability negotiation version
    /// specified by the client, or else 0 if the
    /// client did not specify a CAP version.
    pub fn caps_version(&self) -> u16 {
        self.caps_version
    }

    /// Set the capability negotiation version only
    /// if the new version is greater than the old
    /// version.
    pub fn set_caps_version(&mut self, new: u16) -> u16 {
        if new > self.caps_version {
            self.caps_version = new;
        }
        
        self.caps_version
    }

    /// Borrow the internal ping deadline tracker.
    /// If None, then the server is not waiting on any
    /// PONG reply from the client.
    /// If Some, then the client is registered as inactive
    /// and must reply with a PONG and matching u64 token
    /// by `Instant` or else have its connecton terminated.
    pub fn ping_deadline(&mut self) -> &mut Option<(Instant, u64)> {
        &mut self.ping_deadline
    }
}
