use std::{env, fs::File, io::BufReader};
use serde::{Deserialize, Serialize};
use serde_json;
use clap::Parser;

mod exec; // look in exec.rs
mod environment;    // look in environment.rs

// Configuration struct, populated with serde_json and clap.
#[derive(Parser)]
#[command(author, version, about)]
#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    /// The package manager to execute
    #[arg(short, long)]
    exe: Option<String>,

    /// The directory to chroot to
    #[arg(short, long)]
    root_dir: Option<String>,

    /// A list of enviornment variables that the package manager needs
    #[arg(short, long)]
    keep_env: Option<Vec<String>>,
}

impl Config {
    fn overlay(mut self, other: Config) -> Self {
        self.keep_env = match self.keep_env {
            Some(mut k) => {
                match other.keep_env {
                    Some(mut l) => {
                        k.append(&mut l);
                        k.sort();
                        k.dedup();
                        Some(k)
                    },
                    None => Some(k),
                }
            },
            None => {
                match other.keep_env {
                    Some(k) => Some(k),
                    None => None,
                }
            }
        };

        Self {
            exe: other.exe.or(self.exe),
            root_dir: other.root_dir.or(self.root_dir),
            keep_env: self.keep_env,
        }
    }


}



fn from_filename(fname: &str) -> Option<Config> {
     match File::open(fname) {
        Err(_) => return None,
        Ok(f) => {
            let reader = BufReader::new(f);
            match serde_json::from_reader(reader) {
                Err(e) => {
                    panic!("Error parsing {}: {}", fname, e);
                }
                Ok(config) => {
                    return Some(config);
                }
            };
        },
    };
}



//fn parse_config() -> SafePkgConfig {
//    
//    let data = r#"{
//            "exe": "/usr/bin/printenv",
//            "keep_env": [
//                "HTTPS_PROXY",
//                "HTTP_PROXY",
//                "HOME",
//                "USER"
//            ],
//            "root_dir": "/app1"
//        }"#;
//    let v = match serde_json::from_str::<SafePkgConfig>(data) {
//        Ok(v) => v,
//        Err(e) => {
//            eprintln!("Failed to parse config: {e}");
//            exit(1);
//        }
//
//    };
//    v
//}

fn main() {

    // Program defaults
    let mut config = Config{
        exe: None,
        root_dir:  Some(String::from("/")),
        keep_env: Some([].to_vec())
    };
    let etc_filename = String::from("/etc/safe-package/config.json");
    let user_filename = match env::var("HOME") {
        // Most unixen
        Ok(val) => format!("{val}/.safe-package/config.json"),
        // Some single-user embedded systems
        Err(_e) => String::from("/.safe-package/config.json"),
    };

    let cwd_filename = String::from("./.safe-package/config.json");

    // This will be our end result configuration.
    // let config = defaults.overlay(defaults);

    // Parse whole args with clap
    // let mut args = Args::parse();

    config = match from_filename(&etc_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };

    config = match from_filename(&user_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };

    config = match from_filename(&cwd_filename) {
        None => config,
        Some(c) => config.overlay(c),
    };

    config = config.overlay(Config::parse());

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
