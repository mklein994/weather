#[macro_use]
extern crate clap;
extern crate darksky;
extern crate drawille;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate tokio_core;
extern crate weather_icons;

#[cfg(feature = "local")]
extern crate serde_json;

use clap::ArgMatches;
use darksky::{Block, DarkskyHyperRequester, Language, Unit};
use drawille::Canvas;
use futures::Future;
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use std::env;
use tokio_core::reactor::Core;
#[cfg(feature = "local")]
pub use local::run;

use darksky::models::Icon as DarkskyIcon;
use weather_icons::Icon;

pub mod app;
#[cfg(feature = "local")]
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

#[cfg(not(feature = "local"))]
pub fn run(config: Config, matches: ArgMatches) -> Result<(), Box<std::error::Error>> {
    info!("using remote");

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
    let d = weather.daily.unwrap();
    let h = weather.hourly.unwrap();

    let degrees = "°";

    let mut output = format!(
        "{current_temp}{degrees} {summary}. ({feels_like_temp}{degrees})",
        degrees = degrees,
        current_temp = c.temperature.unwrap().round(),
        summary = c.summary.unwrap(),
        feels_like_temp = c.apparent_temperature.unwrap().round()
    );

    let hourly_temperatures = get_hourly_temperature(h.data.unwrap());
    let temperature_graph = graph(hourly_temperatures);

    if m.is_present("i3") {
        let icon_string = format!(
            "<span font_desc='Weather Icons'>{icon}</span>",
            icon = get_icon(&c.icon.unwrap())
        );

        let moon = format!(
            "<span font_desc='Weather Icons'>{}</span>",
            weather_icons::moon::phase(d.data.unwrap()[0].moon_phase.unwrap())
        );

        output = [icon_string, output, moon].join(" ");
    }

    println!("{} {}", temperature_graph, output);

    if m.is_present("long") {
        println!("{}", h.summary.unwrap());
        println!("{}", d.summary.unwrap());
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

fn get_hourly_temperature(datapoints: Vec<darksky::models::Datapoint>) -> Vec<Option<f32>> {
    let mut wind = Vec::with_capacity(64);

    for h in datapoints {
        wind.push(h.temperature.map_or(None, |t| Some(t as f32)));
    }

    info!("capacity: {:?}", wind.capacity());
    wind
}

fn graph(datapoints: Vec<Option<f32>>) -> String {
    let mut canvas = Canvas::new(datapoints.capacity() as u32, 3);
    let mut debug_canvas = canvas.clone();

    let max = datapoints
        .iter()
        .max_by(|a, b| a.unwrap().partial_cmp(&b.unwrap()).unwrap())
        .unwrap()
        .unwrap();
    let min = datapoints
        .iter()
        .min_by(|a, b| a.unwrap().partial_cmp(&b.unwrap()).unwrap())
        .unwrap()
        .unwrap();

    // A braille character (eg \u28FF (⣿)), can have up to 8 dots, and each are numbered.
    // Here's the hex table:
    //  1 **  8
    //  2 ** 10
    //  3 ** 20
    // 40 ** 80
    //
    // The Unicode value is calculated by adding the sum of each part to 2800. For example,
    //
    // 0 *
    // * 0
    // 0 0
    // 0 0
    //
    // is calculated as 2 + 8 + 2800 = \u280A.
    //
    // Including blanks to count as the maximum and minimum, the entire range looks like this:
    // '⠀⡠⠊⠀'
    //TODO: fix documentation.
    for (i, d) in datapoints.iter().enumerate() {
        let raw_amount;
        let amount;
        if d.is_none() {
            raw_amount = min.round() as u32;
            amount = 4;
        } else {
            raw_amount = ((max.abs() + min.abs()) - (d.unwrap() + min.abs())).round() as u32;
            // (a, b):     min/max of range
            // (min, max): min/max from list
            // x:          some value from the list
            //
            //        (b-a)(x - min)
            // f(x) = -------------- + a
            //          max - min
            amount = 4 - (((5. - 0.) * (d.unwrap() - min)) / (max - min) + 0.).round() as i32;
        };

        debug_canvas.set(i as u32, raw_amount);
        debug_canvas.set(i as u32, max.abs().round() as u32);

        if amount >= 0 && amount <= 3 {
            canvas.set(i as u32, amount as u32);
        }
    }
    info!("debug_canvas:\n{}", debug_canvas.frame());
    canvas.frame()
}
