extern crate darksky;
extern crate dotenv;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate weather_icons;

extern crate serde_json;

use darksky::{Block, DarkskyHyperRequester, Language, Unit};
use futures::Future;
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::Core;

use darksky::models::Icon as DarkskyIcon;
use weather_icons::Icon;

mod local;

#[derive(Debug)]
pub struct Config {
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

    if let Err(e) = if env::var("DARKSKY_LOCAL").is_ok() {
        local::run()
    } else {
        run(config)
    } {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}

pub fn print_weather(weather: darksky::models::Forecast) {
    println!("{:#?}", weather);
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
            print_weather(f);
            Ok(())
        });

    core.run(work)?;
    Ok(())
}

fn get_icon(icon: &DarkskyIcon) -> Icon {
    match *icon {
        DarkskyIcon::ClearDay => Icon::DarkskyClearDay,
        DarkskyIcon::ClearNight => Icon::DarkskyClearNight,
        DarkskyIcon::Cloudy => Icon::DarkskyCloudy,
        DarkskyIcon::Fog => Icon::DarkskyFog,
        DarkskyIcon::Hail => Icon::DarkskyHail,
        DarkskyIcon::PartlyCloudyDay => Icon::DarkskyPartlyCloudyDay,
        DarkskyIcon::PartlyCloudyNight => Icon::DarkskyPartlyCloudyNight,
        DarkskyIcon::Rain => Icon::DarkskyRain,
        DarkskyIcon::Sleet => Icon::DarkskySleet,
        DarkskyIcon::Snow => Icon::DarkskySnow,
        DarkskyIcon::Thunderstorm => Icon::DarkskyThunderstorm,
        DarkskyIcon::Tornado => Icon::DarkskyTornado,
        DarkskyIcon::Wind => Icon::DarkskyWind,
    }
}
