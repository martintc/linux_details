pub mod distro;
pub mod family;
pub mod init;
pub mod package_manager;

#[cfg(not(target_os = "linux"))]
compile_error!("This crate is only supported on Linux");
