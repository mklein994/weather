use std::fmt;

pub const SPARKS_FONT_SIZE: usize = 100;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Weight {
    ExtraSmall,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

impl Default for Weight {
    fn default() -> Self {
        Weight::Medium
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Style {
    Bar,
    Dot,
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
        Self { style, weight }
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
