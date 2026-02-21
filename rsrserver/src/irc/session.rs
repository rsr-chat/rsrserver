use tokio::time::Instant;
use crate::irc::Capabilities;

pub struct IrcSession {
    caps_version: u16,
    caps: Capabilities,

    ping_deadline: Option<(Instant, u64)>,
}

impl Default for IrcSession {
    fn default() -> Self {
        IrcSession {
            caps_version: 0,
            caps: Capabilities::empty(),
            ping_deadline: None,
        }
    }
}

impl IrcSession {
    pub fn caps(&self) -> &Capabilities {
        &self.caps
    }

    pub fn caps_mut(&mut self) -> &mut Capabilities {
        &mut self.caps
    }

    pub fn caps_version(&self) -> u16 {
        self.caps_version
    }

    pub fn set_caps_version(&mut self, new: u16) -> u16 {
        if new > self.caps_version {
            self.caps_version = new;
        }
        
        self.caps_version
    }

    pub fn ping_deadline(&mut self) -> &mut Option<(Instant, u64)> {
        &mut self.ping_deadline
    }
}

pub mod state {
    #[derive(Debug, Default)]
    pub struct Anonymous {
        pub nick: Option<String>,
        pub user: Option<String>,
        pub real: Option<String>,
    }
}

