use darksky;
use serde_json;
use toml;

use Error::*;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Darksky(darksky::Error),
    Io(io::Error),
    Json(serde_json::Error),
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Darksky(ref err) => write!(f, "Darksky error: {}", err),
            Io(ref err) => write!(f, "IO error: {}", err),
            Json(ref err) => write!(f, "Serde JSON error: {}", err),
            Toml(ref err) => write!(f, "Toml deserialize error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Darksky(ref err) => err.description(),
            Io(ref err) => err.description(),
            Json(ref err) => err.description(),
            Toml(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Darksky(ref err) => Some(err),
            Io(ref err) => Some(err),
            Json(ref err) => Some(err),
            Toml(ref err) => Some(err),
        }
    }
}

impl From<darksky::Error> for Error {
    fn from(err: darksky::Error) -> Self {
        Darksky(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        use serde_json::error::Category;
        match err.classify() {
            Category::Io => Io(err.into()),
            Category::Syntax | Category::Data | Category::Eof => Json(err),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Toml(err)
    }
}
