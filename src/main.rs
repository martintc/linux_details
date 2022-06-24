use linux_details::*;

fn main() {
    match linux_details::distro::Distro::get_distro_info() {
        Some(distro) => {
            println!("{}", distro.get_type());
            println!("{}", distro.get_init());
            println!("{}", distro.get_init());
            println!("{}", distro.get_package_manager());
        },
        None => {
            println!("An issue detecting a distro");
        }
    }
}
