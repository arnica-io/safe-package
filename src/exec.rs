use nix::unistd::execv;
use std::ffi::CString;
use std::process::exit;
//use std::env;

pub fn exec_pm(path: &str, args: Vec<&str>) {
        
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
