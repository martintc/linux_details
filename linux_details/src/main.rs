use linux_details::distro::Distro;

fn main() {
    match Distro::get_distro_info() {
        Some(distro) => {
            println!("{}", distro.get_type());
            println!("{}", distro.get_init());
            println!("{}", distro.get_package_manager());
            println!("{}", distro.get_family());
        }
        None => {
            println!("An issue detecting a distro");
        }
    }
}