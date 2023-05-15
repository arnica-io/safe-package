use std::env;
use clap::Parser;
use std::process::exit;
use debug_print::debug_println;
use nix::unistd;



mod exec;           // look in exec.rs
mod environment;    // look in environment.rs
mod config;         // look in config.rs
mod chroot;         // look in chroot.rs

fn main() {

    // Program defaults
    let mut config = config::Config{
        exe: None,
        root_dir:  Some(String::from("/")),
        keep_env: Some([].to_vec()),
        user: None,
        exe_args: [].to_vec(),
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

    debug_println!("{:?}", config);

    match config.keep_env {
        None => { 
            environment::clear_env(&[ ].to_vec());
        },
        Some(k) => {
            environment::clear_env(&k);
        },
    }

    match config.root_dir {
        None => { }, // Nothing to do.
        Some(d) => {
            if d != (String::from("/")) {
                match chroot::chroot(&d) {
                    Ok(()) => { },
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    },
                }
            }
        },
    }

    match config.user {
        None => {
                if unistd::geteuid().is_root() {
                    println!("Warning! Running as root. I hope you know what you're doing.");
                }
        }, 
        Some(user) => {
            match exec::drop_privs(&user) {
                Ok(()) => { },
                Err(e) => {
                    eprintln!("Couldn't drop privileges to user {}: {}",
                        user, e);
                    exit(1);

                },
            }
        },
    }

    match config.exe {
        Some(e) => { 
           exec::exec_pm(&e, config.exe_args.to_vec());
        },
        None => {
            if config.exe_args.len() > 0 {
                let exe = config.exe_args[0].clone();
                config.exe_args = config.exe_args[1..].to_vec();
                exec::exec_pm(&exe, config.exe_args.to_vec());
            } else {
                panic!("Nothing to execute!");
            }
        },
    }
}
