use darksky;
use serde_json;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub fn run() -> Result<(), Box<::std::error::Error>> {
    let mut contents = String::new();
    let path = env::var("DARKSKY_LOCAL").unwrap();
    File::open(path).and_then(|mut file| {
        file.read_to_string(&mut contents)?;
        let weather: darksky::models::Forecast = serde_json::from_str(&contents)?;
        super::print_weather(weather);
        Ok(())
    })?;
    Ok(())
}