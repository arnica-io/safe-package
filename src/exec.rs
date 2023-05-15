use nix::unistd::{execv, setuid};
use std::ffi::CString;
use std::process::exit;
use nix::unistd::User;

//use std::env;

pub fn drop_privs(user: &str) -> Result<(),&'static str> {
    match User::from_name(user) {
        Ok(o) => { 
            match o {
                None => Err("user not found"),
                Some(u) => {
                    match setuid(u.uid) {
                        Err(_e) => Err("Failed to setuid. Are you root?"),
                        _ => Ok(()),
                    }
                },
            }
        },
        Err(_) => Err("Failed to setuid. Are you root?"),
    }
}

    
pub fn exec_pm(path: &str, args: Vec<std::string::String>) {
        
    let p = &CString::new(path).unwrap();
    let mut v = Vec::new();
    v.push(p.clone());

    for arg in args {
        v.push(CString::new(arg).unwrap());
    }
    match execv(p, &v) {
        Err(e) => {
            eprintln!("Failed to execute {path}: {e}");
        },
        Ok(_) => eprintln!("The impossible has happened."),
    };

    exit(1);

}
