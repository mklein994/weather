use std::fmt;

pub const SPARKS_FONT_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Weight {
    #[serde(rename = "extra-small")]
    ExtraSmall,
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "extra-large")]
    ExtraLarge,
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Medium
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Style {
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "dot")]
    Dot,
    #[serde(rename = "dot-line")]
    DotLine,
}

impl Default for Style {
    fn default() -> Self {
        Style::Bar
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
pub struct Font {
    pub style: Style,
    pub weight: Weight,
}

impl Font {
    pub fn new(style: Style, weight: Weight) -> Self {
        Font { style, weight }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Style::*;
        use self::Weight::*;
        write!(
            f,
            "Sparks {}",
            match self.style {
                Bar => format!(
                    "Bar{}",
                    match self.weight {
                        ExtraSmall => "Extra-narrow",
                        Small => "Narrow",
                        Medium => "Medium",
                        Large => "Wide",
                        ExtraLarge => "Extra-wide",
                    }
                ),
                Dot => format!(
                    "Dot{}",
                    match self.weight {
                        ExtraSmall => "Extra-small",
                        Small => "Small",
                        Medium => "Medium",
                        Large => "Large",
                        ExtraLarge => "Extra-large",
                    }
                ),
                DotLine => format!(
                    "Dot-line{}",
                    match self.weight {
                        ExtraSmall => "Extra-thin",
                        Small => "Thin",
                        Medium => "Medium",
                        Large => "Thick",
                        ExtraLarge => "Extra-thick",
                    }
                ),
            }
        )
    }
}
