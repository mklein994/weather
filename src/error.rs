use clap;
use darksky;
use serde_json;
use toml;

use crate::Error::*;
use std::error;
use std::fmt;
use std::io;
use weather_icons::OutOfBounds;

#[derive(Debug)]
pub enum Error {
    Clap(clap::Error),
    Darksky(darksky::Error),
    Io(io::Error),
    Json(serde_json::Error),
    Moon(OutOfBounds),
    Toml(toml::de::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Clap(ref err) => err.fmt(f),
            Darksky(ref err) => err.fmt(f),
            Io(ref err) => err.fmt(f),
            Json(ref err) => err.fmt(f),
            Moon(ref err) => err.fmt(f),
            Toml(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Clap(err)
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

impl From<OutOfBounds> for Error {
    fn from(err: OutOfBounds) -> Self {
        Moon(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Toml(err)
    }
}
