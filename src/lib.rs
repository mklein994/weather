extern crate ansi_term;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate darksky;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate spark;
extern crate toml;
extern crate weather_icons;

pub mod app;
pub mod color;
pub mod config;
pub mod error;
pub mod graph;

use chrono::{DateTime, Local, TimeZone, Timelike};
use clap::ArgMatches;
use darksky::{Block, DarkskyReqwestRequester, Language, Unit};
use darksky::models::Icon as DarkskyIcon;
use reqwest::Client;
use std::fs::File;
use std::io::prelude::*;
use weather_icons::{moon, Icon};

pub use config::Config;
use error::WeatherError;
use graph::Graph;

type Result<T> = std::result::Result<T, WeatherError>;

pub fn run(config: &Config, matches: &ArgMatches) -> Result<()> {
    let weather_data = get_weather(&config, &matches)?;
    print_weather(matches, config, weather_data);

    Ok(())
}

fn get_weather(config: &Config, matches: &ArgMatches) -> Result<darksky::models::Forecast> {
    if matches.is_present("debug") {
        let mut contents = String::new();
        let path = config.local.clone().unwrap();
        info!("using local file: {}", path);
        let mut f = File::open(path)?;
        f.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).map_err(WeatherError::Json)
    } else {
        let client = Client::new();
        match matches.occurrences_of("historical") {
            0 => client
                .get_forecast_with_options(&config.token, config.lat, config.lon, |o| {
                    o.exclude(vec![Block::Minutely])
                        .unit(Unit::Auto)
                        .language(Language::En)
                })
                .map_err(WeatherError::Darksky),
            _ => client
                .get_forecast_time_machine(
                    &config.token,
                    config.lat,
                    config.lon,
                    matches.value_of("historical").unwrap(),
                    |o| {
                        o.exclude(vec![Block::Minutely])
                            .unit(Unit::Auto)
                            .language(Language::En)
                    },
                )
                .map_err(WeatherError::Darksky),
        }
    }
}

pub fn print_weather(matches: &ArgMatches, config: &Config, weather: darksky::models::Forecast) {
    let c = weather.currently.unwrap();
    let d = weather.daily.unwrap();
    let h = weather.hourly.unwrap();

    let hourly_data = h.data.unwrap();
    let daily_data = d.data.unwrap();

    let degrees = "°";

    let mut output = format!(
        "{current_temp}{degrees} {summary}. ({feels_like_temp}{degrees})",
        degrees = degrees,
        current_temp = c.temperature.unwrap().round(),
        summary = c.summary.unwrap(),
        feels_like_temp = c.apparent_temperature.unwrap().round()
    );

    let pressures: Vec<Option<f64>> = hourly_data.iter().map(|d| d.pressure).collect();

    let times: Vec<DateTime<Local>> = hourly_data
        .iter()
        .map(|d| Local.timestamp(d.time as i64, 0))
        .collect();

    let mut pressure_graph = Graph::new();
    pressure_graph.values(&pressures);

    let position = find_closest_time_position(&Local.timestamp(c.time as i64, 0), &times);
    info!("calculated position: {:?}", position);

    if let Some(ref h) = config.highlight {
        pressure_graph.highlight(&position, h);
    }

    if let Some(ref f) = config.font {
        pressure_graph.font(f);
    }

    debug!("pressure graph: {:?}", pressure_graph);
    debug!("pressure graph sparkline: {:?}", pressure_graph.sparkline());
    debug!("pressure graph sparkfont: {:?}", pressure_graph.sparkfont());

    let daily_temperatures: Vec<Option<f64>> = daily_data
        .iter()
        .map(|d| {
            debug!("daily high temp: {}", d.temperature_high.unwrap());
            d.temperature_high
        })
        .collect();

    let (daily_temperature_min, daily_temperature_max, temperature_spark_graph) =
        spark::graph_opt(&daily_temperatures);

    if matches.is_present("i3") {
        // TODO: put pressure sparkfont graph here

        let icon_string = format!(
            "<span font_desc='Weather Icons'>{icon}</span>",
            icon = get_icon(&c.icon.unwrap())
        );

        let moon = format!(
            "<span font_desc='Weather Icons'>{}</span>",
            weather_icons::moon::phase(moon::Color::Primary, daily_data[0].moon_phase.unwrap())
        );

        output = [pressure_graph.sparkfont(), icon_string, output, moon].join(" ");
    }

    println!("{}", output);

    if matches.is_present("long") {
        println!("hourly pressure forecast:\n{}", pressure_graph.sparkline());
        println!(
            "temperatures this week:\n{} {} {}",
            daily_temperature_min, temperature_spark_graph, daily_temperature_max
        );
        println!(
            "{}",
            h.summary.unwrap_or_else(|| "no hourly summary".to_owned())
        );
        println!(
            "{}",
            d.summary.unwrap_or_else(|| "no daily summary".to_owned())
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

fn find_closest_time_position(time: &DateTime<Local>, times: &[DateTime<Local>]) -> Option<usize> {
    let current_time = time;
    times
        .iter()
        .inspect(|t| {
            if current_time.date() == time.date() && current_time.hour() == t.hour() {
                debug!("current_time: {:?}, t: {:?}", current_time, t)
            }
        })
        .position(|time| current_time.date() == time.date() && current_time.hour() == time.hour())
}
