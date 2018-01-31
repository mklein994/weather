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
use weather_icons::{Condition, Icon, Moon, Style, Time};

pub use config::Config;
use error::WeatherError;
use graph::Graph;

type Result<T> = std::result::Result<T, WeatherError>;

pub fn run(config: &Config, matches: &ArgMatches) -> Result<()> {
    let weather_data = get_weather(&config, &matches)?;
    if matches.occurrences_of("json") == 1 {
        println!(
            "{}",
            serde_json::to_string_pretty(&weather_data)
                .expect("couldn't convert weather data back to json")
        );
    } else {
        print_weather(matches, config, weather_data);
    }

    Ok(())
}

fn get_weather(config: &Config, matches: &ArgMatches) -> Result<darksky::models::Forecast> {
    if matches.is_present("debug") || matches.is_present("local") {
        let mut contents = String::new();

        let path = if let Some(p) = matches.value_of("local") {
            p.to_string()
        } else {
            config.local.clone().unwrap()
        };

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

    let (sunrise, sunset) = (
        Local.timestamp(daily_data[0].sunrise_time.unwrap() as i64, 0),
        Local.timestamp(daily_data[0].sunset_time.unwrap() as i64, 0),
    );

    if matches.is_present("i3") {
        let pressure_icon = format!("<span font_desc='Weather Icons'>{}</span>", Icon::Barometer);

        let current_condition_icon = format!(
            "<span font_desc='Weather Icons'>{icon}</span>",
            icon = get_current_condition_icon(&c.icon.unwrap(), &Local::now(), &sunrise, &sunset)
        );

        let wind_bearing_icon = get_wind_bearing_icon(c.wind_bearing.unwrap().trunc() as u32);

        let moon = format!(
            "<span font_desc='Weather Icons'>{}</span>",
            Moon::new()
                .style(Style::Primary)
                .phase(daily_data[0].moon_phase.unwrap())
                .unwrap()
                .build()
        );

        output = [
            pressure_icon,
            pressure_graph.sparkfont(),
            current_condition_icon,
            output,
            format!("<span font_desc='Fira Code'>{}</span>", wind_bearing_icon),
            format!("{} km/h", c.wind_speed.unwrap().round() as i32),
            moon,
        ].join(" ");
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

fn get_current_condition_icon(
    icon: &DarkskyIcon,
    now: &DateTime<Local>,
    sunrise: &DateTime<Local>,
    sunset: &DateTime<Local>,
) -> Icon {
    let time = if *now >= *sunrise && *now <= *sunset {
        Time::Day
    } else {
        Time::Night
    };

    let new_icon = match *icon {
        DarkskyIcon::Tornado => Some(Icon::Tornado),
        DarkskyIcon::Wind => Some(Icon::Windy),
        _ => match *icon {
            DarkskyIcon::ClearNight | DarkskyIcon::ClearDay => Condition::Fair,
            DarkskyIcon::Cloudy => Condition::Cloudy,
            DarkskyIcon::Fog => Condition::Fog,
            DarkskyIcon::Hail => Condition::Hail,
            DarkskyIcon::PartlyCloudyNight | DarkskyIcon::PartlyCloudyDay => {
                Condition::PartlyCloudy
            }
            DarkskyIcon::Rain => Condition::Rain,
            DarkskyIcon::Sleet => Condition::Sleet,
            DarkskyIcon::Snow => Condition::Snow,
            DarkskyIcon::Thunderstorm => Condition::Thunderstorm,
            _ => unreachable!(),
        }.variant(time),
    };

    new_icon.unwrap_or(Default::default())
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

fn get_wind_bearing_icon<'a>(bearing: u32) -> &'a str {
    let arrows = vec![
        "\u{2191}", // (↑) north
        "\u{2197}", // (↗) north-east
        "\u{2192}", // (→) east
        "\u{2198}", // (↘) south-east
        "\u{2193}", // (↓) south
        "\u{2199}", // (↙) south-west
        "\u{2190}", // (←) west
        "\u{2196}", // (↖) north-west
    ];

    // The wind bearing is given by the direction it's coming from, so flip it around to point in
    // the direction it's blowing to.
    //
    // 360 degrees divided by 8 cardinal points = 45,
    // shifted over by 45/2 = 22.5,
    // truncated.
    // TODO: don't hard code these ranges.
    match (bearing + 180) % 360 {
        338...360 | 0...22 => arrows[0],
        23...67 => arrows[1],
        68...112 => arrows[2],
        113...157 => arrows[3],
        158...202 => arrows[4],
        203...247 => arrows[5],
        248...292 => arrows[6],
        293...337 => arrows[7],
        _ => "wind bearing out of range",
    }
}
