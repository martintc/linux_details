use os_info::{Bitness, Type, Version};

use crate::{family::Family, init::Init, package_manager::PackageManager};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Distro {
    os_type: Type,
    version: Version,
    bitness: Bitness,
    init: Init,
    supported_package_managers: Vec<PackageManager>,
    available_package_managers: Vec<PackageManager>,
    default_package_manager: PackageManager,
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
            supported_package_managers: vec![],
            available_package_managers: vec![],
            default_package_manager: PackageManager::Unknown,
            family: Family::Unknown,
        }
    }

    pub fn get_distro_info() -> Self {
        let info = os_info::get();

        Distro {
            os_type: info.os_type(),
            version: info.version().clone(),
            bitness: info.bitness(),
            init: Init::get_init(),
            supported_package_managers: vec![], // PackageManager::supported_package_managers(info.os_type())
            available_package_managers: vec![], // PackageManager::available_package_manager(info.os_type())
            default_package_manager: PackageManager::Unknown, // PackageManager::default_package_manager(info.os_type())
            family: Family::family_of_os_type(info.os_type()),
        }
    }

    #[inline]
    pub fn get_type(&self) -> Type {
        self.os_type
    }

    #[inline]
    pub fn get_version(&self) -> &Version {
        &self.version
    }

    #[inline]
    pub fn get_bitness(&self) -> Bitness {
        self.bitness
    }

    #[inline]
    pub fn get_init(&self) -> Init {
        self.init
    }

    #[inline]
    pub fn get_supported_package_managers(&self) -> &[PackageManager] {
        &self.supported_package_managers
    }

    #[inline]
    pub fn get_available_package_managers(&self) -> &[PackageManager] {
        &self.available_package_managers
    }

    #[inline]
    pub fn get_default_package_manager(&self) -> PackageManager {
        self.default_package_manager
    }

    #[inline]
    pub fn get_family(&self) -> Family {
        self.family
    }
}
