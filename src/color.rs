use ::std::str::FromStr;

#[derive(Debug, Default, Deserialize)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f64,
}

impl Color {
    pub fn hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}{:02x}",
            self.red,
            self.green,
            self.blue,
            (self.alpha * 255.).round() as u8,
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
            alpha: f64::from(u8::from_str_radix(&color[7..], 16)?),
        };
        Ok(c)
    }
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        format!("{:08x}", c).parse().unwrap()
    }
}
