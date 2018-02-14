#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate weather;

use clap::Shell;
use std::env;
use std::path::PathBuf;
use weather::{app, Config};

fn main() {
    env_logger::try_init().expect("failed to initialize logger");

    let matches = app::build_cli().get_matches();
    debug!("app matches: {:#?}", matches);

    if let ("completions", Some(completion_matches)) = matches.subcommand() {
        debug!("completion matches: {:#?}", completion_matches);

        let shell = completion_matches
            .value_of("shell")
            .and_then(|s| s.parse::<Shell>().ok())
            .expect("couldn't parse shell name");

        info!("shell: {:?}", shell);

        app::build_cli().gen_completions_to(crate_name!(), shell, &mut std::io::stdout());
    } else {
        let settings_path = matches.value_of("config").map_or(
            env::home_dir()
                .expect("couldn't determine home directory")
                .join(".config")
                .join(env!("CARGO_PKG_NAME"))
                .join("config.toml"),
            PathBuf::from,
        );

        let config = Config::from_path_buf(&settings_path).unwrap_or_else(|e| {
            //TODO: handle as a proper error
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
}
