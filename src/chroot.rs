use std::os::unix::fs;
use nix::unistd;

pub fn chroot(path: &str) -> Result<(),&'static str> {

    if ! unistd::geteuid().is_root() {
        return Err("You must be root to set a root-dir. Configure a 'user' to drop privs after chrooting.");
    }

    fs::chroot(path).expect("Failed to chroot");
    
    std::env::set_current_dir("/").expect("Failed to change directory");

    Ok(())
}
    
