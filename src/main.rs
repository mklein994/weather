extern crate darksky;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use darksky::{Block, DarkskyHyperRequester, Language, Unit};
use futures::Future;
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::Core;

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
    let mut core = Core::new()?;
    let handle = core.handle();

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle)?)
        .build(&handle);

    let work = client
        .get_forecast_with_options(&config.token, config.lat, config.lon, |o| {
            o.exclude(vec![Block::Minutely])
                .unit(Unit::Auto)
                .language(Language::En)
        })
        .and_then(move |f| {
            println!("{:#?}", f);
            Ok(())
        });

    core.run(work)?;
    Ok(())
}
