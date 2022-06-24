use stf::fmt::{self, Display, Formatter};
use os_info::{Type, Version, Bitness};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distro {
    os_type: os_info::Type,
    version: os_info::Version,
    bitness: os_info::Bitness,
    init: Init,
    package_manager: PackageManager,
    family: Family,
}

impl Distro {
    pub fn unknown() -> Self {
        let info = os_info::get();
        Distro {
            os_type: info.os_type(),
            version: info.version(),
            bitness: info.bitness(),
            init: Init::Unknown,
            package_manager: PackageManager::Unknown,
        }
    }

    pub fn get_type(&self) -> os_info::Type {
        self.os_type
    }

    pub fn get_version(&self) -> os_info::Version {
        self.version
    }

    pub fn get_bitness(&self) -> os_info::Bitness {
        self.bitness
    }

    pub fn get_init(&self) -> Init {
        self.init
    }

    pub fn get_package_managert(&self) -> PackageManager {
        self.package_manager
    }

    fn is_linux() -> bool {
        let info = os_info::get();
        matches!(info.os_type, Type::Amazon, Type::Alpine, Type::Android, Type::Arch, Type::CentOS, Type::Debian, Type::EndeavourOS, Type::Fedora, Type::Gentoo, Type::Linux, Type::Manjaro, Type::Mint, Type::NixOS, Type::openSUSE, Type::Redhat, Type::Pop, Type::OracleLinux, Type::RedHatEnterprise, Type::Solus, Type::SUSE, Type::Ubuntu)
    }
}