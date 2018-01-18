use color::Color;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use super::{Result, WeatherError};
use toml;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub token: String,
    pub lat: f64,
    pub lon: f64,
    pub bg_color: Color,
    pub fg_color: Color,
    pub local: Option<String>,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        toml::from_str(&contents).map_err(WeatherError::Toml)
    }
}
