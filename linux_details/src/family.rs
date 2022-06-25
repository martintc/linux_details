use enum_iterator::Sequence;

use linux_details_macros::Family;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Family, Sequence)]
pub enum Family {
    #[default_variant]
    Unknown,
    #[display_name("arch-based")]
    #[os_types(Arch, EndeavourOS, Manjaro)]
    ArchBased,
    #[display_name("debian-based")]
    #[os_types(Debian, Raspbian)]
    DebianBased,
    #[display_name("gentoo-based")]
    #[os_types(Gentoo)]
    GentooBased,
    #[os_types(Alpine, Android, Linux, NixOS, Solus)]
    Independent,
    #[display_name("redhat-based")]
    #[os_types(Amazon, CentOS, Fedora, OracleLinux, RedHatEnterprise, Redhat)]
    RedhatBased,
    #[display_name("slackware-based")]
    SlackwareBased,
    #[display_name("suse-based")]
    #[os_types(openSUSE, SUSE)]
    SuseBased,
    #[display_name("ubuntu-based")]
    #[os_types(Ubuntu, Mint, Pop)]
    UbuntuBased,
}

mod tests {
    #[allow(unused_imports)]
    use super::Family;

    #[test]
    fn default() {
        let package_manager = Family::default();
        assert_eq!(package_manager, Family::Unknown);
    }

    #[test]
    fn display() {
        macro_rules! check_if_str {
            ($($name:ident => $str:expr),* $(,),*) => {
                $(
                    assert_eq!(
                        format!("{}", Family::$name).as_str(),
                        $str
                    );
                )*
            };
        }

        check_if_str![
            ArchBased => "arch-based",
            DebianBased => "debian-based",
            GentooBased => "gentoo-based",
            Independent => "independent",
            RedhatBased => "redhat-based",
            SlackwareBased => "slackware-based",
            SuseBased => "suse-based",
            UbuntuBased => "ubuntu-based",
        ];
    }

    #[test]
    fn get_package_manager() {
        macro_rules! family_check {
            ($(
                $family:ident => [$($os_type:ident),+]
            ),* $(,)*) => {
                $($(
                    assert_eq!(Family::get_family(os_info::Type::$os_type), Family::$family);
                )*)*
            };
        }

        family_check![
            ArchBased => [Arch, EndeavourOS, Manjaro],
            DebianBased => [Debian, Raspbian],
            GentooBased => [Gentoo],
            Independent => [Alpine, Android, Linux, NixOS, Solus],
            RedhatBased => [Amazon, CentOS, Fedora, OracleLinux, RedHatEnterprise, Redhat],
            SuseBased => [openSUSE, SUSE],
            UbuntuBased => [Ubuntu, Mint, Pop],
        ];
    }
}
