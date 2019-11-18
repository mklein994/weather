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
        Self::Medium
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
        Self::Bar
    }
}

#[derive(Debug, Default, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_font_display_string_bar() {
        assert_eq!(
            "Sparks BarExtra-narrow",
            Font::new(Style::Bar, Weight::ExtraSmall).to_string()
        );
        assert_eq!(
            "Sparks BarNarrow",
            Font::new(Style::Bar, Weight::Small).to_string()
        );
        assert_eq!(
            "Sparks BarMedium",
            Font::new(Style::Bar, Weight::Medium).to_string()
        );
        assert_eq!(
            "Sparks BarWide",
            Font::new(Style::Bar, Weight::Large).to_string()
        );
        assert_eq!(
            "Sparks BarExtra-wide",
            Font::new(Style::Bar, Weight::ExtraLarge).to_string()
        );
    }

    #[test]
    fn check_font_display_string_dot() {
        assert_eq!(
            "Sparks DotExtra-small",
            Font::new(Style::Dot, Weight::ExtraSmall).to_string()
        );
        assert_eq!(
            "Sparks DotSmall",
            Font::new(Style::Dot, Weight::Small).to_string()
        );
        assert_eq!(
            "Sparks DotMedium",
            Font::new(Style::Dot, Weight::Medium).to_string()
        );
        assert_eq!(
            "Sparks DotLarge",
            Font::new(Style::Dot, Weight::Large).to_string()
        );
        assert_eq!(
            "Sparks DotExtra-large",
            Font::new(Style::Dot, Weight::ExtraLarge).to_string()
        );
    }

    #[test]
    fn check_font_display_string_bar_dotline() {
        assert_eq!(
            "Sparks Dot-lineExtra-thin",
            Font::new(Style::DotLine, Weight::ExtraSmall).to_string()
        );
        assert_eq!(
            "Sparks Dot-lineThin",
            Font::new(Style::DotLine, Weight::Small).to_string()
        );
        assert_eq!(
            "Sparks Dot-lineMedium",
            Font::new(Style::DotLine, Weight::Medium).to_string()
        );
        assert_eq!(
            "Sparks Dot-lineThick",
            Font::new(Style::DotLine, Weight::Large).to_string()
        );
        assert_eq!(
            "Sparks Dot-lineExtra-thick",
            Font::new(Style::DotLine, Weight::ExtraLarge).to_string()
        );
    }
}
