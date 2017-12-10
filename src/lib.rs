#[macro_use]
extern crate clap;
extern crate darksky;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate weather_icons;

extern crate serde_json;

use clap::ArgMatches;
use darksky::{Block, DarkskyHyperRequester, Language, Unit};
use futures::Future;
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::Core;

use darksky::models::Icon as DarkskyIcon;
use weather_icons::Icon;

pub mod app;
pub mod local;

#[derive(Debug)]
pub struct Config {
    token: String,
    lat: f64,
    lon: f64,
}

impl Config {
    pub fn new() -> Self {
        Config {
            token: env::var("DARKSKY_KEY").unwrap(),
            lat: env::var("DARKSKY_LAT").unwrap().parse::<f64>().unwrap(),
            lon: env::var("DARKSKY_LON").unwrap().parse::<f64>().unwrap(),
        }
    }
}

pub fn run(config: Config, matches: ArgMatches) -> Result<(), Box<std::error::Error>> {
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
        .and_then(move |weather| {
            print_weather(matches, weather);
            Ok(())
        });

    core.run(work)?;
    Ok(())
}

pub fn print_weather(m: ArgMatches, weather: darksky::models::Forecast) {
    let c = weather.currently.unwrap();

    let icon = get_icon(&c.icon.unwrap());

    let summary = c.summary.unwrap();
    let current_temp = c.temperature.unwrap();
    let feels_like_temp = c.apparent_temperature.unwrap();

    let degrees = "°";

    if m.is_present("i3") {
        println!(
            "<span font_desc='Weather Icons'>{icon}</span> {summary}: {current_temp}{degrees} ({feels_like_temp}{degrees})",
            icon = icon,
            degrees = degrees,
            summary = summary,
            current_temp = current_temp,
            feels_like_temp = feels_like_temp
            );
    } else {
        println!(
            "{summary}: {current_temp}{degrees} ({feels_like_temp}{degrees})",
            summary = summary,
            degrees = degrees,
            current_temp = current_temp,
            feels_like_temp = feels_like_temp
        );
    }
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
