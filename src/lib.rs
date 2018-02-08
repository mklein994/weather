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
extern crate toml;
extern crate weather_icons;

pub mod app;
pub mod color;
mod config;
mod error;
pub mod graph;

use chrono::{DateTime, Local, TimeZone, Timelike};
use clap::ArgMatches;
use darksky::{Block, DarkskyReqwestRequester, Language, Unit};
use darksky::models::Icon as DarkskyIcon;
use reqwest::Client;
use std::fs::File;
use std::io::prelude::*;
use weather_icons::{Condition, DripIcon, Icon, Moon, Style, Time};

pub use config::Config;
pub use error::Error;
use graph::Graph;

type Result<T> = std::result::Result<T, Error>;

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
        serde_json::from_str(&contents).map_err(Error::Json)
    } else {
        let client = Client::new();
        match matches.occurrences_of("historical") {
            0 => client
                .get_forecast_with_options(
                    &config.token,
                    if matches.is_present("latitude") {
                        matches
                            .value_of("latitude")
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                    } else {
                        config.lat
                    },
                    if matches.is_present("longitude") {
                        matches
                            .value_of("longitude")
                            .unwrap()
                            .parse::<f64>()
                            .unwrap()
                    } else {
                        config.lon
                    },
                    |o| {
                        o.exclude(vec![Block::Minutely])
                            .unit(Unit::Ca)
                            .language(Language::En)
                    },
                )
                .map_err(Error::Darksky),
            _ => client
                .get_forecast_time_machine(
                    &config.token,
                    config.lat,
                    config.lon,
                    matches.value_of("historical").unwrap(),
                    |o| {
                        o.exclude(vec![Block::Minutely])
                            .unit(Unit::Ca)
                            .language(Language::En)
                    },
                )
                .map_err(Error::Darksky),
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
        summary = c.summary.clone().unwrap(),
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

    if let Some(ref f) = config.font_style {
        info!("{:?}", f);
        pressure_graph.font.style = *f;
    }

    if let Some(ref w) = config.font_weight {
        pressure_graph.font.weight = *w;
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

    let daily_temperature_spark_graph = Graph::new().values(&daily_temperatures).sparkline();

    let (sunrise, sunset) = (
        Local.timestamp(daily_data[0].sunrise_time.unwrap() as i64, 0),
        Local.timestamp(daily_data[0].sunset_time.unwrap() as i64, 0),
    );

    if matches.is_present("i3") {
        let pressure_icon = format!("<span font_desc='Weather Icons'>{}</span>", Icon::Barometer);

        /*
         * clear             clear
         *
         * light-clouds      partly cloudy
         * heavy-clouds      overcast
         *
         * very-light-rain   drizzle
         * light-rain        light rain
         * medium-rain       rain
         * heavy-rain        heavy rain
         *
         * very-light-sleet  light sleet
         * light-sleet       light sleet
         * medium-sleet      sleet
         * heavy-sleet       heavy sleet
         *
         * very-light-snow   flurries
         * light-snow        light snow
         * medium-snow       snow
         * heavy-snow        heavy snow
         */
        /*
        let precipitation = Precipitation::new(
            &c.precip_probability.unwrap_or(0.),
            &c.precip_intensity.unwrap_or(0.),
            &c.precip_accumulation,
            &c.precip_type,
        );
        */

        /*
        let precipitation = if *precip_probability == 0. {
            None
        } else {
            Precipitation::new(
                &c.precip_probability,
                &c.precip_intensity,
                &c.precip_accumulation,
                &c.precip_type,
            )
        };
        */

        debug!("{:?}", &c.icon.unwrap());

        let current_condition_icon = format!(
            //"<span font_desc='Weather Icons'>{icon}</span>",
            "<span font_desc='dripicons-weather'>{icon}</span>",
            icon = DripIcon::from(get_current_condition_icon(
                //icon = get_current_condition_icon(
                &c.icon.unwrap(),
                //&precipitation,
                &Local::now(),
                &sunrise,
                &sunset
            ))
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
        println!("temperatures this week:\n{}", daily_temperature_spark_graph);
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

/*
/*
probability: f64,
intensity: f64,
accumulation: Option<f64>,
precip_type: Option<darksky::models::PrecipitationType>,
*/
struct Precipitation {}

impl Precipitation {
    fn new(
        probability: &f64,
        intensity: &f64,
        accumulation: &Option<f64>,
        precip_type: &Option<darksky::models::PrecipitationType>,
    ) -> Option<Self> {
        use darksky::models::PrecipitationType;

        if *probability == 0. || *intensity == 0. {
            return None
        } else {
            if let Some(pt) = precip_type {
                match *pt {
                    PrecipitationType::Rain | PrecipitationType::Sleet => unimplemented!(),
                    PrecipitationType::Snow => unimplemented!(),
                }
            }
        }

        /*
        let probability = (*probability)?;
        if let Some(p) = precip_type.and_then(|p| p == PrecipitationType::Snow) {
            unimplemented!()
        }
        //debug_assert!(*intensity >= 0. && *intensity <= 1.);
        */

        unimplemented!()
    }
}
*/

fn get_current_condition_icon(
    icon: &DarkskyIcon,
    //precipitation: &Option<Precipitation>,
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

    new_icon.unwrap_or_default()
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
    info!("current wind bearing: {}°", (bearing + 180) % 360);

    let arrows = vec![
        "\u{2197}", // (↗) north-east
        "\u{2192}", // (→) east
        "\u{2198}", // (↘) south-east
        "\u{2193}", // (↓) south
        "\u{2199}", // (↙) south-west
        "\u{2190}", // (←) west
        "\u{2196}", // (↖) north-west
        "\u{2191}", // (↑) north
    ];

    // The wind bearing describes the direction of the wind source, make it point to where
    // it's blowing.
    let bearing = (bearing + 180) % 360;

    // Starting from the end of the list, find the closest arrow compared to the bearing.
    //
    // Note that the ratio to convert an arrow to a degree is 360 divided the number of
    // arrows.
    //
    // For every arrow:
    //
    //   - multiply each index by tha ratio to convert to degrees
    //   - add half of the ratio to shift the capturing range over a bit. For example, if
    //   there are 8 arrows, south would include 157° to 202°, passing through
    //   direct south (180°).
    if let Some((_, arrow)) = arrows
        .iter()
        .enumerate()
        .rev()
        // This is just the simplified version of what is described earlier.
        .find(|&(i, _)| bearing as usize > (360 * i + 180) / arrows.len())
    {
        arrow
    } else {
        // Because we've adjusted the range, this won't catch bearings between 337°
        // and 22°, so assume this is pointing north.
        arrows.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wind_bearing() {
        let arrows = vec![
            "\u{2197}", // (↗) north-east
            "\u{2192}", // (→) east
            "\u{2198}", // (↘) south-east
            "\u{2193}", // (↓) south
            "\u{2199}", // (↙) south-west
            "\u{2190}", // (←) west
            "\u{2196}", // (↖) north-west
            "\u{2191}", // (↑) north
        ];

        for bearing in 0..360 {
            let expected = match (bearing + 180) % 360 {
                23...67 => arrows[0],
                68...112 => arrows[1],
                113...157 => arrows[2],
                158...202 => arrows[3],
                203...247 => arrows[4],
                248...292 => arrows[5],
                293...337 => arrows[6],
                338...360 | 0...22 => arrows[7],
                _ => unreachable!()//"wind bearing out of range",
            };

            let actual = get_wind_bearing_icon(bearing);

            println!("{:?}, {:?}, {:?}", bearing, expected, actual);
            assert_eq!(expected, actual);
        }
    }
}
