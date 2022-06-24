use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Init {
    Launchd,
    Openrc,
    Runnit,
    Upstart,
    Unknown,
    S6,
    Systemd,
    SysV,
}

impl Default for Init {
    fn default() -> Self {
        Init::Unknown
    }
}

impl Display for Init {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Init::Launchd => write!(f, "launchd"),
            Init::Openrc => write!(f, "openrc"),
            Init::Runnit => write!(f, "runnit"),
            Init::Upstart => write!(f, "upstart"),
            Init::S6 => write!(f, "s6"),
            Init::Systemd => write!(f, "systemd"),
            Init::SysV => write!(f, "sysv"),
            _ => write!(f, "{:?}", self),
        }
    }
}