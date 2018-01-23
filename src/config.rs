use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use super::{Result, WeatherError};
use toml;
use graph::{Font, Highlight};

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub token: String,
    pub lat: f64,
    pub lon: f64,
    pub font: Option<Font>,
    pub highlight: Option<Highlight>,
    pub local: Option<String>,
}

impl Config {
    pub fn from_path_buf(path: &PathBuf) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        toml::from_str(&contents).map_err(WeatherError::Toml)
    }
}
