use std::fmt::{self, Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PackageManager {
    Aptitude,
    Dnf,
    Eopkg,
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
            PackageManager::Aptitude => write!(f, "aptitde"),
            PackageManager::Dnf => write!(f, "dnf"),
            PackageManager::Eopkg => write!(f, "eopkg"),
            PackageManager::Pacman => write!(f, "pacman"),
            PackageManager::Portage => write!(f, "portage"),
            PackageManager::Yum => write!(f, "yum"),
            PackageManager::Zypper => write!(f, "zypper"),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl PackageManager {
    pub fn get_package_manager() -> PackageManager {
        PackageManager::Unknown
    }
}