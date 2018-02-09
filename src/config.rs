use super::{Error, Result};
use graph::{Highlight, Style, Weight};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

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
}

impl Config {
    pub fn from_path_buf(path: &PathBuf) -> Result<Self> {
        let mut f = File::open(path)?;

        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        toml::from_str(&contents).map_err(Error::Toml)
    }
}
