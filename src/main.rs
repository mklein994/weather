#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate weather;

use clap::Shell;
use weather::{app, Config, Result};

fn main() {
    if let Err(e) = parse_arguments() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn parse_arguments() -> Result<()> {
    env_logger::try_init()?;

    let matches = app::build_cli().get_matches();
    debug!("app matches: {:#?}", matches);

    if let ("completions", Some(completion_matches)) = matches.subcommand() {
        debug!("completion matches: {:#?}", completion_matches);

        let shell = value_t!(completion_matches.value_of("shell"), Shell)?;

        info!("shell: {:?}", shell);

        app::build_cli().gen_completions_to(crate_name!(), shell, &mut std::io::stdout());

        return Ok(());
    }

    let config = Config::new(matches)?;

    weather::run_new(config)

    /*

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
        */
}
