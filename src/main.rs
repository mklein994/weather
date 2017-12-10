extern crate dotenv;
extern crate weather;

use std::env;
use weather::{app, local, Config};

fn main() {
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

    if let Err(e) = if env::var("DARKSKY_LOCAL").is_ok() {
        local::run(matches)
    } else {
        weather::run(config, matches)
    } {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
