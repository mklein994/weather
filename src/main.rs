extern crate dotenv;

use std::env;

#[derive(Debug)]
struct Config {
    token: String,
    lat: f64,
    lon: f64,
}

impl Config {
    fn new() -> Self {
        Config {
            token: env::var("DARKSKY_KEY").unwrap(),
            lat: env::var("DARKSKY_LAT").unwrap().parse::<f64>().unwrap(),
            lon: env::var("DARKSKY_LON").unwrap().parse::<f64>().unwrap(),
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let config = Config::new();

    if let Err(e) = run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<std::error::Error>> {
    println!("config: {:?}", config);
    Ok(())
}
