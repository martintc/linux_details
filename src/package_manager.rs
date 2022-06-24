use std::fmt::{self, Display, Formatter};
use os_info::Type;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageManager {
    Apk,
    Aptitude,
    Dnf,
    Eopkg,
    Nix,
    Pacman,
    Portage,
    Unknown,
    Yum,
    Zypper,
}

impl Default for PackageManager {
    fn default() -> Self {
        PackageManager::Unknown
    }
}

impl Display for PackageManager {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            PackageManager::Apk => write!(f, "apk"),
            PackageManager::Aptitude => write!(f, "aptitde"),
            PackageManager::Dnf => write!(f, "dnf"),
            PackageManager::Eopkg => write!(f, "eopkg"),
            PackageManager::Nix => write!(f, "nix"),
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Portage => write!(f, "portage"),
            PackageManager::Yum => write!(f, "yum"),
            PackageManager::Zypper => write!(f, "zypper"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl PackageManager {
    pub fn get_package_manager(os_type: os_info::Type) -> PackageManager {
        match os_type {
            Type::Alpine => PackageManager::Apk,
            Type::Amazon => PackageManager::Yum,
            Type::Android => PackageManager::Unknown,
            Type::Arch => PackageManager::Pacman,
            Type::CentOS => PackageManager::Dnf,
            Type::Debian => PackageManager::Aptitude,
            Type::EndeavourOS => PackageManager::Pacman,
            Type::Fedora => PackageManager::Dnf,
            Type::Gentoo => PackageManager::Portage,
            Type::Linux => PackageManager::Unknown,
            Type::Manjaro => PackageManager::Pacman,
            Type::Mint => PackageManager::Aptitude,
            Type::NixOS => PackageManager::Nix,
            Type::openSUSE => PackageManager::Zypper,
            Type::OracleLinux => PackageManager::Yum,
            Type::Pop => PackageManager::Aptitude,
            Type::Raspbian => PackageManager::Aptitude,
            Type::RedHatEnterprise => PackageManager::Yum,
            Type::Redhat => PackageManager::Yum,
            Type::Solus => PackageManager::Eopkg,
            Type::SUSE => PackageManager::Zypper,
            Type::Ubuntu => PackageManager::Aptitude,
            _ => PackageManager::Unknown,
        }
    }
}