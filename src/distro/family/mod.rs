use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Family {
    ArchBased,
    DebianBased,
    GentooBased,
    Independent,
    RedhatBased,
    SlackwareBased,
    SuseBased,
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
            Family::GentooBased => write!(f, "gentoo-based"),
            Family::Independent => write!(f, "independent"),
            Family::RedhatBased => write!(f, "redhat-based"),
            Family::SlackwareBased => write!(f, "slackware-based"),
            Family::SuseBased => write!(f, "suse-based"),
            Family::UbunutBased => write!(f, "ubuntu-based"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Family {
    pub fn get_family(os_type: os_info::Type) -> Family {
        match os_type {
            Type::Alpine => Family::Independent,
            Type::Amazon => Family::RedhatBased,
            Type::Android => Family::Independent,
            Type::Arch => Family::ArchBased,
            Type::CentOs => Family::RedhatBased,
            Type::Debian => Family::Debian,
            Type::EndeavourOS => Family::ArchBased,
            Type::Fedora => Family::RedhatBased,
            Type::Gentoo => Family::GentooBased,
            Type::Linux => Family::Independent,
            Type::Manjaro => Family::ArchBased,
            Type::Mint => Family::UbuntuBased,
            Type::NixOS => Family::Independent,
            Type::openSUSE => Family::SuseBased,
            Type::OracleLinux => Family::RedhatBased,
            Type::Pop => Family::Ubuntu,
            Type::Raspbian => Family::DebianBased,
            Type::Redhat => Family::RedhatBased,
            Type::RedHatEnterprise => Family::RedhatBased,
            Type::Solus => Family::Independent,
            Type::SUSE => Family::SuseBased,
            Type::Ubuntu => Family::UbuntuBased,
            Type::Garuda => Family::ArchBased,
            _ => Family::Independent,
        }
    }
}

