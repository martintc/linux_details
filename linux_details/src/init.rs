use enum_iterator::Sequence;

use linux_details_macros::LinuxDetailsEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, LinuxDetailsEnum, Sequence)]
pub enum Init {
    #[default_variant]
    Unknown,
    Launchd,
    Openrc,
    Runnit,
    Upstart,
    S6,
    Systemd,
    SysV,
}

impl Init {
    pub fn get_init() -> Self {
        // TODO: need to implement a way to get the init system of the current OS
        Self::default()
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::Init;

    #[test]
    fn default() {
        let init = Init::default();
        assert_eq!(init, Init::Unknown);
    }

    #[test]
    fn display() {
        for variant in enum_iterator::all::<Init>() {
            assert_eq!(
                format!("{:?}", variant).to_lowercase().as_str(),
                format!("{}", variant).as_str()
            );
        }
    }

    #[test]
    fn get_init() {
        let package_manager = Init::get_init();
        assert_eq!(package_manager, Init::Unknown);
    }
}
