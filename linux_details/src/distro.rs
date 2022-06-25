use os_info::{Bitness, Type, Version};

use crate::{family::Family, init::Init, package_manager::PackageManager};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distro {
    os_type: Type,
    version: Version,
    bitness: Bitness,
    init: Init,
    package_manager: PackageManager,
    family: Family,
}

impl Distro {
    pub fn unknown() -> Self {
        let info = os_info::get();

        Distro {
            os_type: info.os_type(),
            version: info.version().clone(),
            bitness: info.bitness(),
            init: Init::Unknown,
            package_manager: PackageManager::Unknown,
            family: Family::Unknown,
        }
    }

    pub fn get_distro_info() -> Option<Self> {
        if !is_linux() {
            return None;
        }
        let info = os_info::get();
        Some(Distro {
            os_type: info.os_type(),
            version: info.version().clone(),
            bitness: info.bitness(),
            init: Init::get_init(),
            package_manager: PackageManager::get_package_manager(info.os_type()),
            family: Family::get_family(info.os_type()),
        })
    }

    pub fn get_type(&self) -> Type {
        self.os_type
    }

    pub fn get_version(&self) -> &Version {
        &self.version
    }

    pub fn get_bitness(&self) -> Bitness {
        self.bitness
    }

    pub fn get_init(&self) -> Init {
        self.init
    }

    pub fn get_package_manager(&self) -> PackageManager {
        self.package_manager
    }

    pub fn get_family(&self) -> Family {
        self.family
    }
}
