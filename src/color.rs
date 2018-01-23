use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};

#[derive(Copy, Clone, Debug, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.red, self.green, self.blue, self.alpha,
        )
    }

    pub fn rgba(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha,
        )
    }
}

impl FromStr for Color {
    type Err = ::std::num::ParseIntError;
    fn from_str(color: &str) -> Result<Self, Self::Err> {
        let c = Self {
            red: u8::from_str_radix(&color[1..3], 16)?,
            green: u8::from_str_radix(&color[3..5], 16)?,
            blue: u8::from_str_radix(&color[5..7], 16)?,
            alpha: u8::from_str_radix(&color[7..], 16)?,
        };
        Ok(c)
    }
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        format!("{:08x}", c).parse().unwrap()
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}
