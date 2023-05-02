use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json;

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


