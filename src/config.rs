use super::{Error, Result};
use crate::graph::{Highlight, Style, Weight};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;
use weather_icons::Style as MoonStyle;

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
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
    // Represents `pub moon_style: Option<MoonStyle>`
    #[serde(with = "MoonStyleRemote", default = "Default::default")]
    pub moon_style: MoonStyle,
    pub icon_style: Option<IconStyle>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IconStyle {
    WeatherIcons,
    Dripicons,
}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle::WeatherIcons
    }
}
