use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Family {
    ArchBased,
    DebianBased,
    GentooBased,
    RedhatBased,
    SlackwareBased,
    UbuntuBased,
    Unknown,
}

impl Default for Family {
    fn default() -> Self {
        Family::Unknown
    }
}

impl Display for Family {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Family::ArchBased => write!(f, "arch-based"),
            Family::DebianBased => write!(f, "debian-based"),
            Family::GentooBased => Write!(f, "gentoo-based"),
            Family::RedhatBased => write!(f, "redhat-based"),
            Family::SlackwareBased => write!(f, "slackware-based"),
            Family::UbunutBased => write!(f, "ubuntu-based"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Family {
    pub fn get_family() -> Family {
        
    }
}

