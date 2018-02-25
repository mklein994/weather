use super::{Error, Result};
use graph::{Highlight, Style, Weight};
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
    #[serde(with = "MoonStyleRemote", default = "Default::default")]
    pub moon_style: MoonStyle,
}

impl Config {
    pub fn from_path_buf(path: &PathBuf) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        toml::from_str(&contents).map_err(Error::Toml)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(remote = "MoonStyle", rename_all = "kebab-case")]
enum MoonStyleRemote {
    Primary,
    Alt,
}
