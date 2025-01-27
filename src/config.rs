use std::env;
use std::process;
use lazy_static::lazy_static;

use crate::*;

pub const DATA_DIR: &str = "/var/tmp";
pub const BIND_ADDRESS: &str = "0.0.0.0:8001";

#[derive(Debug)]
pub struct Config {
    pub verbose: bool,
    pub data_dir: String,
    pub bind_addr: String,
    pub args: Vec<String>,
}

lazy_static! {

    // Command line configuration
    pub static ref CONFIG: Config = Config::cmdline();
}

// perhaps make a global structure of above so that it can
// be referred to from anywhere in the namespace without an
// instance variable?

/* Chappell's lightweight getopt() for rust */
impl Default for Config {
    fn default() -> Config {
        Config {
            verbose: false,
            data_dir: String::from(""),
            bind_addr: String::from(BIND_ADDRESS),
            args: vec![],
        }
    }
}
impl Config {
    pub fn usage() {
        eprintln!("Usage: ddserver [-v] [-d data-dir] [-l listen address]");
        process::exit(1);
    }

    pub fn cmdline() -> Config {
        let mut config = Config::default();

        let mut args = env::args();
        args.next(); // blow off the first argument
        while let Some(a) = args.next() {
            config.args.push(match a.as_str() {
                "-v" => {
                    config.verbose = true;
                    continue;
                }
                "-d" => {
                    config.data_dir =
                        args.next()
                            .expect("expected data directory");
                    std::env::set_current_dir(&config.data_dir)
                        .expect("cannot use specified data directory");
                    continue;
                },
                "-l" => {
                    config.bind_addr =
                        args.next()
                            .expect("expected address to listen on, eg. 0.0.0.0:8001");
                    continue;
                },
                "-h" => {
                    Self::usage();
                    break;
                }
                _ => {
                    a
                }
            })
        }

        config
    }
}
