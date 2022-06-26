use enum_iterator::Sequence;

use linux_details_macros::PackageManager;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, PackageManager, Sequence)]
pub enum PackageManager {
    #[default_variant]
    Unknown,
    #[os_types(Alpine)]
    Apk,
    #[os_types(Debian, Mint, Pop, Raspbian, Ubuntu)]
    Aptitude,
    #[os_types(CentOS, Fedora)]
    Dnf,
    #[os_types(Solus)]
    Eopkg,
    #[os_types(NixOS)]
    Nix,
    #[os_types(Arch, EndeavourOS, Manjaro)]
    Pacman,
    #[os_types(Gentoo)]
    Portage,
    #[os_types(Amazon, OracleLinux, RedHatEnterprise, Redhat)]
    Yum,
    #[os_types(openSUSE, SUSE)]
    Zypper,
}

mod tests {
    #[allow(unused_imports)]
    use super::PackageManager;

    #[test]
    fn default() {
        let package_manager = PackageManager::default();
        assert_eq!(package_manager, PackageManager::Unknown);
    }

    #[test]
    fn display() {
        for variant in enum_iterator::all::<PackageManager>() {
            assert_eq!(
                format!("{:?}", variant).to_lowercase().as_str(),
                format!("{}", variant).as_str()
            );
        }
    }

    #[test]
    fn get_package_manager() {
        macro_rules! package_manager_check {
            ($(
                $package_manager:ident => [$($os_type:ident),+]
            ),* $(,)*) => {
                $($(
                    assert_eq!(PackageManager::get_package_manager(os_info::Type::$os_type), PackageManager::$package_manager);
                )*)*
            };
        }

        package_manager_check![
            Apk => [Alpine],
            Aptitude => [Debian, Mint, Pop, Raspbian, Ubuntu],
            Dnf => [CentOS, Fedora],
            Eopkg => [Solus],
            Nix => [NixOS],
            Pacman => [Arch, EndeavourOS, Manjaro],
            Portage => [Gentoo],
            Yum => [Amazon, OracleLinux, RedHatEnterprise, Redhat],
            Zypper => [openSUSE, SUSE],
        ];
    }
}
