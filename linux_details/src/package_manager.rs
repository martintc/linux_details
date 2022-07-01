use enum_iterator::Sequence;

use linux_details_macros::LDEnum;

// TODO: do bit more research on this part to set proper defaults
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, LDEnum, Sequence)]
pub enum PackageManager {
    #[ld_enum_conf(is_default_variant)]
    Unknown,
    #[ld_enum_conf(os_types(Alpine), default_in_os_types(Alpine))]
    Apk,
    #[ld_enum_conf(
        os_types(Debian, Mint, Pop, Raspbian, Ubuntu),
        default_in_os_types(Debian, Mint, Pop, Raspbian, Ubuntu)
    )]
    Apt,
    #[ld_enum_conf(os_types(Debian, Mint, Pop, Raspbian, Ubuntu))]
    Aptitude,
    #[ld_enum_conf(os_types(CentOS, Fedora), default_in_os_types(CentOS, Fedora))]
    Dnf,
    #[ld_enum_conf(
        os_types(Amazon, OracleLinux, RedHatEnterprise, Redhat),
        default_in_os_types(Amazon, OracleLinux, RedHatEnterprise, Redhat)
    )]
    Yum,
    #[ld_enum_conf(os_types(openSUSE, SUSE), default_in_os_types(openSUSE, SUSE))]
    Zypper,
    #[ld_enum_conf(os_types(Solus), default_in_os_types(Solus))]
    Eopkg,
    #[ld_enum_conf(os_types(NixOS), default_in_os_types(NixOS))]
    Nix,
    #[ld_enum_conf(
        os_types(Arch, EndeavourOS, Manjaro),
        default_in_os_types(Arch, EndeavourOS, Manjaro)
    )]
    Pacman,
    #[ld_enum_conf(os_types(Gentoo), default_in_os_types(Gentoo))]
    Portage,

    #[ld_enum_conf(os_types(
        Alpine,
        Arch,
        CentOS,
        Debian,
        EndeavourOS,
        Fedora,
        Gentoo,
        Manjaro,
        Mint,
        NixOS,
        openSUSE,
        SUSE,
        Pop,
        Raspbian,
        Solus,
        Ubuntu
    ))]
    Flatpak,
    #[ld_enum_conf(os_types(
        Arch,
        CentOS,
        Debian,
        EndeavourOS,
        Fedora,
        Manjaro,
        Mint,
        openSUSE,
        SUSE,
        Pop,
        Solus,
        Ubuntu,
        RedHatEnterprise,
        Redhat
    ))]
    Snap,
}

/*mod tests {
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
        todo!("Make the test to make sure everything is not fucked")
    }
}*/
