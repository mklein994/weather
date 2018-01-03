#[macro_use]
extern crate clap;
extern crate darksky;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde_json;
extern crate spark;
extern crate stats;
extern crate weather_icons;

use clap::ArgMatches;
use darksky::{Block, DarkskyReqwestRequester, Language, Unit};
use reqwest::Client;
use error::WeatherError;
use std::env;
use std::fs::File;
use std::io::prelude::*;

use darksky::models::Icon as DarkskyIcon;
use weather_icons::Icon;
use weather_icons::moon::Color;

type Result<T> = std::result::Result<T, WeatherError>;

mod error {
    use darksky;
    use serde_json;
    use std::error;
    use std::fmt;
    use std::io;

    #[derive(Debug)]
    pub enum WeatherError {
        Darksky(darksky::Error),
        Io(io::Error),
        Json(serde_json::Error),
    }

    impl fmt::Display for WeatherError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                WeatherError::Darksky(ref err) => write!(f, "Darksky error: {}", err),
                WeatherError::Io(ref err) => write!(f, "IO error: {}", err),
                WeatherError::Json(ref err) => write!(f, "Serde JSON error: {}", err),
            }
        }
    }

    impl error::Error for WeatherError {
        fn description(&self) -> &str {
            match *self {
                WeatherError::Darksky(ref err) => err.description(),
                WeatherError::Io(ref err) => err.description(),
                WeatherError::Json(ref err) => err.description(),
            }
        }

        fn cause(&self) -> Option<&error::Error> {
            match *self {
                WeatherError::Darksky(ref err) => Some(err),
                WeatherError::Io(ref err) => Some(err),
                WeatherError::Json(ref err) => Some(err),
            }
        }
    }

    impl From<darksky::Error> for WeatherError {
        fn from(err: darksky::Error) -> Self {
            WeatherError::Darksky(err)
        }
    }

    impl From<io::Error> for WeatherError {
        fn from(err: io::Error) -> Self {
            WeatherError::Io(err)
        }
    }

    impl From<serde_json::Error> for WeatherError {
        fn from(err: serde_json::Error) -> Self {
            use serde_json::error::Category;
            match err.classify() {
                Category::Io => WeatherError::Io(err.into()),
                Category::Syntax | Category::Data | Category::Eof => WeatherError::Json(err),
            }
        }
    }
}

pub mod app;

#[derive(Debug, Default)]
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

pub fn run(config: &Config, matches: &ArgMatches) -> Result<()> {
    let weather_data = get_weather(&config, &matches)?;
    print_weather(matches, weather_data);

    Ok(())
}

fn get_weather(config: &Config, matches: &ArgMatches) -> Result<darksky::models::Forecast> {
    if matches.is_present("debug") {
        let mut contents = String::new();
        let path = env::var("DARKSKY_LOCAL").unwrap();
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

pub fn print_weather(matches: &ArgMatches, weather: darksky::models::Forecast) {
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

    let mut stats: Vec<stats::OnlineStats> = Vec::new();
    let mut s = stats::OnlineStats::new();
    let hourly_pressures: Vec<Option<f64>> = hourly_data
        .iter()
        .map(|d| {
            match d.pressure {
                Some(p) => s.add(p),
                None => s.add_null(),
            };
            debug!("pressure: {:?}", d.pressure);
            stats.push(s);
            d.pressure
        })
        .collect();

    let pressure_spark_graph = spark::graph_opt(&hourly_pressures);

    let daily_temperatures: Vec<Option<f64>> = daily_data
        .iter()
        .map(|d| {
            debug!("temp: {}", d.temperature_high.unwrap());
            d.temperature_high
        })
        .collect();

    let temperature_spark_graph = spark::graph_opt(&daily_temperatures);

    if matches.is_present("i3") {
        let pressure_smooth_graph = spark::smooth_graph(&hourly_pressures);

        let icon_string = format!(
            "<span font_desc='Weather Icons'>{icon}</span>",
            icon = get_icon(&c.icon.unwrap())
        );

        let moon = format!(
            "<span font_desc='Weather Icons'>{}</span>",
            weather_icons::moon::phase(Color::Primary, daily_data[0].moon_phase.unwrap())
        );

        output = [
            format!("<span font_desc='Graph'>{}</span>", pressure_smooth_graph),
            icon_string,
            output,
            moon,
        ].join(" ");
    }

    println!("{}", output);

    if matches.is_present("long") {
        println!("hourly pressure forecast:\n{}", pressure_spark_graph);
        println!("temperatures this week:\n{}", temperature_spark_graph);
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
