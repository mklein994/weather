use darksky;
use serde_json;
use toml;

use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum WeatherError {
    Darksky(darksky::Error),
    Io(io::Error),
    Json(serde_json::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WeatherError::Darksky(ref err) => write!(f, "Darksky error: {}", err),
            WeatherError::Io(ref err) => write!(f, "IO error: {}", err),
            WeatherError::Json(ref err) => write!(f, "Serde JSON error: {}", err),
            WeatherError::Toml(ref err) => write!(f, "Toml deserialize error: {}", err),
        }
    }
}

impl error::Error for WeatherError {
    fn description(&self) -> &str {
        match *self {
            WeatherError::Darksky(ref err) => err.description(),
            WeatherError::Io(ref err) => err.description(),
            WeatherError::Json(ref err) => err.description(),
            WeatherError::Toml(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            WeatherError::Darksky(ref err) => Some(err),
            WeatherError::Io(ref err) => Some(err),
            WeatherError::Json(ref err) => Some(err),
            WeatherError::Toml(ref err) => Some(err),
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

impl From<toml::de::Error> for WeatherError {
    fn from(err: toml::de::Error) -> Self {
        WeatherError::Toml(err)
    }
}
