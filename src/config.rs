use super::Result;
use clap::ArgMatches;
use graph::{Highlight, Style, Weight};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use weather_icons::Style as MoonStyle;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub token: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "font")]
    pub font_style: Option<Style>,
    #[serde(rename = "weight")]
    pub font_weight: Option<Weight>,
    pub highlight: Option<Highlight>,
    pub local: Option<String>,
    pub local_new: Option<PathBuf>,
    #[serde(with = "MoonStyleRemote", default = "Default::default")]
    pub moon_style: MoonStyle,
    settings_path: PathBuf,
    debug: bool,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Result<Self> {
        let path = matches.value_of("config").map_or(
            env::home_dir()
                .expect("couldn't determine home directory")
                .join(".config")
                .join(env!("CARGO_PKG_NAME"))
                .join("config.toml"),
            PathBuf::from,
        );

        let mut f = File::open(&path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let mut config: Config = toml::from_str(&contents)?;

        config.settings_path = path;

        config.local_new = matches.value_of("local").map(|l| PathBuf::from(l));

        info!("{:#?}", config);

        Ok(config)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "MoonStyle")]
enum MoonStyleRemote {
    #[serde(rename = "primary")]
    Primary,
    #[serde(rename = "alt")]
    Alt,
}
