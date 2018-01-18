extern crate dotenv;
extern crate env_logger;
extern crate weather;

use std::env;
use std::path::PathBuf;
use weather::{app, Config};

fn main() {
    env_logger::try_init().expect("failed to initialize logger");

    let matches = app::build_cli();

    let settings_path = matches.value_of("config").map_or_else(
        || {
            env::home_dir()
                .unwrap()
                .join(".config")
                .join(env!("CARGO_PKG_NAME"))
                .join("config.toml")
        },
        |c| PathBuf::from(c),
    );

    let config = Config::from_path(&settings_path.as_path()).unwrap_or_else(|e| {
        eprintln!(
            "Error parsing config file \"{}\": {}",
            settings_path.display(),
            e
        );
        std::process::exit(1);
    });

    if let Err(e) = weather::run(&config, &matches) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}
