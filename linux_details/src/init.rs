use enum_iterator::Sequence;

use linux_details_macros::Init;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Init, Sequence)]
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
