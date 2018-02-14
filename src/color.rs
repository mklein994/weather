use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum CloudColors {
    Clear = 0xeeeef5,
    PartlyCloudy = 0xd5ae2,
    Overcast = 0xb6bfcb,
}

impl CloudColors {
    pub fn color(&self) -> Color {
        Color::from(*self as u32)
    }
}

pub static PRECIPITATION_COLORS: &'static [u32] =
    &[0xeeeef5, 0x96b4da, 0x80a5d6, 0x4a80c7, 0x3267ad];

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
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
        format!("#{:08x}", c).parse().expect("couldn't parse as a color")
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_COLOR: Color = Color {
        red: 0x55,
        green: 0xaa,
        blue: 0xff,
        alpha: 0,
    };
    const TEST_COLOR_STR: &'static str = "#55aaff00";
    const TEST_COLOR_U32: u32 = 0x55_aa_ff_00;

    #[test]
    fn test_color_hex() {
        assert_eq!(TEST_COLOR_STR, TEST_COLOR.hex());
    }

    #[test]
    fn test_color_rgb() {
        assert_eq!("rgba(85, 170, 255, 0)", TEST_COLOR.rgba());
    }

    #[test]
    fn test_color_from_str() {
        assert_eq!(TEST_COLOR_STR.parse::<Color>().unwrap(), TEST_COLOR);
    }

    #[test]
    fn test_color_from_u32() {
        assert_eq!(TEST_COLOR, Color::from(TEST_COLOR_U32));
    }
}
