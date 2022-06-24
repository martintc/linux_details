use os_info::{Type, Version, Bitness};

pub struct Distro {
    os_type: os_info::Type,
    version: os_info::Version,
    bitness: os_info::Bitness,
    init: String,
    package_manager: String,
}