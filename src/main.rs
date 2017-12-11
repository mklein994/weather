extern crate dotenv;
extern crate env_logger;
extern crate weather;

use std::env;
use weather::{app, Config};

fn main() {
    env_logger::init().expect("failed to initialize logger");

    dotenv::from_path(
        env::home_dir()
            .unwrap()
            .join(".config/weather/config")
            .as_path(),
    ).unwrap_or_else(|e| {
        eprintln!("Error parsing config file ~/.config/weather/config: {}", e);
        std::process::exit(1);
    });

    let config = Config::new();
    let matches = app::build_cli();

    if let Err(e) = weather::run(config, matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
