extern crate ansi_term;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate darksky;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate spark;
extern crate stats;
extern crate toml;
extern crate weather_icons;

pub mod app;
pub mod color;
pub mod config;
pub mod error;
pub mod graph;

use chrono::{DateTime, Local, TimeZone, Timelike};
use clap::ArgMatches;
use darksky::models::Icon as DarkskyIcon;
use darksky::{Block, DarkskyReqwestRequester, Language, Unit};
use reqwest::Client;
use std::fs::File;
use std::io::prelude::*;
use weather_icons::{moon, Icon};

pub use config::Config;
use error::WeatherError;
use graph::{Graph, SparkFont, Sparkline};

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
        client
            .get_forecast_with_options(&config.token, config.lat, config.lon, |o| {
                o.exclude(vec![Block::Minutely])
                    .unit(Unit::Auto)
                    .language(Language::En)
            })
            .map_err(WeatherError::Darksky)
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

    let position = find_closest_time_position(&times);

    let pressure_spark_graph = Sparkline::new(&pressures)
        .with_highlight(position, &config.fg_color, &config.bg_color)
        .draw();
    let pressure_font_graph = SparkFont::new(
        pressures.iter().map(|p| p.unwrap_or_else(|| 0.)).collect(),
    ).with_highlight(position, &config.fg_color, &config.bg_color)
        .draw();
    debug!("pressure_spark_graph: {}", pressure_spark_graph);
    debug!("pressure_font_graph: {}", pressure_font_graph);

    let mut stats: Vec<stats::OnlineStats> = Vec::new();
    let mut s = stats::OnlineStats::new();
    let hourly_pressures: Vec<Option<f64>> = hourly_data
        .iter()
        .map(|d| {
            match d.pressure {
                Some(p) => s.add(p),
                None => s.add_null(),
            };
            debug!(
                "pressure:\t{}\t{}",
                Local.timestamp(d.time as i64, 0).to_string(),
                d.pressure.unwrap()
            );
            stats.push(s);
            d.pressure
        })
        .collect();

    let (pressure_min, pressure_max, pressure_spark_graph) = spark::graph_opt(&hourly_pressures);

    let daily_temperatures: Vec<Option<f64>> = daily_data
        .iter()
        .map(|d| {
            debug!("temp: {}", d.temperature_high.unwrap());
            d.temperature_high
        })
        .collect();

    let (daily_temperature_min, daily_temperature_max, temperature_spark_graph) =
        spark::graph_opt(&daily_temperatures);

    if matches.is_present("i3") {
        let pressure_smooth_graph = graph(
            false,
            &hourly_pressures
                .into_iter()
                .filter_map(|p| Some(p.unwrap_or(0.)))
                .collect::<Vec<f64>>(),
        );

        let icon_string = format!(
            "<span font_desc='Weather Icons'>{icon}</span>",
            icon = get_icon(&c.icon.unwrap())
        );

        let moon = format!(
            "<span font_desc='Weather Icons'>{}</span>",
            weather_icons::moon::phase(moon::Color::Primary, daily_data[0].moon_phase.unwrap())
        );

        output = [
            // possible options:
            //  dot-line medium  `spark dot-linemedium`
            //  dot      small   `spark dotsmall`
            //  dot      medium  `spark dotmedium`
            //  bar      medium  `spark barmedium`
            //  bar      narrow  `spark barnarrow`
            //  bar      thin    `spark barthin`
            format!(
                "<span font_desc='spark dotsmall 11'>{}</span>",
                pressure_smooth_graph
            ),
            icon_string,
            output,
            moon,
        ].join(" ");
    }

    println!("{}", output);

    if matches.is_present("long") {
        println!(
            "hourly pressure forecast:\n{} {} {}",
            pressure_min, pressure_spark_graph, pressure_max
        );
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

fn graph(is_dot_line: bool, values: &[f64]) -> String {
    let mut min: f64 = std::f64::MAX;
    let mut max: f64 = 0.;

    for &i in values.iter() {
        if i > max {
            max = i;
        }
        if i < min {
            min = i;
        }
    }

    let ratio = if max == min {
        1.0
    } else {
        let max_tick = if is_dot_line { 9 } else { 100 };

        max_tick as f64 / (max - min)
    };

    format!(
        "{}{{{}}}{}",
        min,
        values
            .iter()
            .cloned()
            .map(|n| (n - min) * ratio)
            .map(|n| n.floor().to_string())
            .collect::<Vec<String>>()
            .join(","),
        max,
    )
}

fn find_closest_time_position(times: &[DateTime<Local>]) -> Option<usize> {
    let current_time = Local::now();
    times.iter().position(|&time| {
        current_time < time && current_time.hour() == time.hour()
    })
}
