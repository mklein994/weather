use color::Color;
use std::env;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub token: String,
    pub lat: f64,
    pub lon: f64,
    pub bg_color: Color,
    pub fg_color: Color,
}

impl Config {
    pub fn new() -> Self {
        Config {
            token: env::var("DARKSKY_KEY").unwrap(),
            lat: env::var("DARKSKY_LAT").unwrap().parse::<f64>().unwrap(),
            lon: env::var("DARKSKY_LON").unwrap().parse::<f64>().unwrap(),
            bg_color: env::var("DARKSKY_BACKGROUND_COLOR").unwrap().parse::<Color>().unwrap(),
            fg_color: env::var("DARKSKY_FOREGROUND_COLOR").unwrap().parse::<Color>().unwrap(),
        }
    }
}
