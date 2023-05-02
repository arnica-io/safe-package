use std::env;
use clap::Parser;

mod exec; // look in exec.rs
mod environment;    // look in environment.rs
mod config; // look in config.rs



fn main() {

    // Program defaults
    let mut config = config::Config{
        exe: None,
        root_dir:  Some(String::from("/")),
        keep_env: Some([].to_vec())
    };

    let etc_filename = String::from("/etc/safe-package/config.json");
    config = match config::from_filename(&etc_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };


    let user_filename = match env::var("HOME") {
        // Most unixen
        Ok(val) => format!("{val}/.safe-package/config.json"),
        // Some single-user embedded systems
        Err(_e) => String::from("/.safe-package/config.json"),
    };
    config = match config::from_filename(&user_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };

    let cwd_filename = String::from("./.safe-package/config.json");
    config = match config::from_filename(&cwd_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };

    config = config.overlay(config::Config::parse());

    println!("{:?}", config);

    match config.keep_env {
        None => { 
            environment::clear_env(&[ ].to_vec());
        },
        Some(k) => {
            environment::clear_env(&k);
        },
    }

    match config.exe {
        None => {
            panic!("No executable defined. Nothing to do. Byee!");
        },
        Some(e) => {
            exec::exec_pm(&e, [].to_vec());
        },
    }
    
}
