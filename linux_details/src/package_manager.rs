use enum_iterator::Sequence;

// TODO: do bit more research on this part to set proper defaults
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, PackageManager, Sequence)]
pub enum PackageManager {
    #[default_variant]
    Unknown,
    #[os_types(Alpine)]
    #[main_in_os_types(Alpine)]
    Apk,
    #[os_types(Debian, Mint, Pop, Raspbian, Ubuntu)]
    #[main_in_os_types(Debian, Mint, Pop, Raspbian, Ubuntu)]
    Apt,
    #[os_types(Debian, Mint, Pop, Raspbian, Ubuntu)]
    Aptitude,
    #[os_types(CentOS, Fedora)]
    #[main_in_os_types(CentOS, Fedora)]
    Dnf,
    #[os_types(Amazon, OracleLinux, RedHatEnterprise, Redhat)]
    #[main_in_os_types(Amazon, OracleLinux, RedHatEnterprise, Redhat)]
    Yum,
    #[os_types(openSUSE, SUSE)]
    #[main_in_os_types(openSUSE, SUSE)]
    Zypper,
    #[os_types(Solus)]
    #[main_in_os_types(Solus)]
    Eopkg,
    #[os_types(NixOS)]
    #[main_in_os_types(NixOS)]
    Nix,
    #[os_types(Arch, EndeavourOS, Manjaro)]
    #[main_in_os_types(Arch, EndeavourOS, Manjaro)]
    Pacman,
    #[os_types(Gentoo)]
    #[main_in_os_types(Gentoo)]
    Portage,

    #[os_types(
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
    )]
    Flatpak,
    #[os_types(
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
    )]
    Snap,
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
        todo!("Make the test to make sure everything is not fucked")
    }
}
