use ansi_term::{self, Style};
use std::fmt;
use std::str::FromStr;

use color::Color;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum Font {
    #[serde(rename = "spark barmedium")]
    BarMedium,
    #[serde(rename = "spark barnarrow")]
    BarNarrow,
    #[serde(rename = "spark barthin")]
    BarThin,
    #[serde(rename = "spark dotmedium")]
    DotMedium,
    #[serde(rename = "spark dotsmall")]
    DotSmall,
    #[serde(rename = "spark dot-linemedium")]
    DotlineMedium,
}

impl Default for Font {
    fn default() -> Self {
        Font::BarMedium
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Font::*;
        write!(
            f,
            "{}",
            match *self {
                BarMedium => "spark barmedium",
                BarNarrow => "spark barnarrow",
                BarThin => "spark barthin",
                DotMedium => "spark dotmedium",
                DotSmall => "spark dotsmall",
                DotlineMedium => "spark dot-linemedium",
            }
        )
    }
}

impl FromStr for Font {
    type Err = String;
    fn from_str(font: &str) -> Result<Self, Self::Err> {
        match font {
            "spark barmedium" => Ok(Font::BarMedium),
            "spark barnarrow" => Ok(Font::BarNarrow),
            "spark barthin" => Ok(Font::BarThin),
            "spark dotmedium" => Ok(Font::DotMedium),
            "spark dotsmall" => Ok(Font::DotSmall),
            "spark dot-linemedium" => Ok(Font::DotlineMedium),
            _ => Err("Could not parse font".to_owned()),
        }
    }
}

impl Font {
    fn size(&self) -> u32 {
        match *self {
            Font::DotlineMedium => 9,
            _ => 100,
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct Highlight {
    pub position: Option<usize>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[derive(Debug, Default)]
pub struct Graph {
    values: Vec<Option<f64>>,
    font: Font,
    pub highlight: Option<Highlight>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            values: Vec::new(),
            font: Default::default(),
            highlight: None,
        }
    }

    pub fn values(&mut self, v: &[Option<f64>]) -> &mut Self {
        self.values = v.to_vec();
        self
    }

    pub fn highlight(&mut self, position: &Option<usize>, highlight: &Highlight) -> &mut Self {
        self.highlight = Some(if !highlight.position.is_some() {
            Highlight {
                position: *position,
                ..*highlight
            }
        } else {
            *highlight
        });
        self
    }

    pub fn font(&mut self, font: &Font) -> &mut Self {
        self.font = *font;
        self
    }

    pub fn sparkline(&self) -> String {
        let bars = "▁▂▃▄▅▆▇█";

        let mut min = ::std::f64::MAX;
        let mut max = 0.;

        self.values.iter().filter_map(|i| *i).for_each(|i| {
            if i > max {
                max = i;
            }
            if i < min {
                min = i;
            }
        });

        let ratio = if min == max {
            1.
        } else {
            (bars.chars().count() - 1) as f64 / (max - min)
        };

        let mut graph = self.values
            .iter()
            .map(|value| {
                if let Some(i) = *value {
                    bars.chars()
                        .nth(((i - min) * ratio).floor() as usize)
                        .expect(&format!("{} is out of bounds", i))
                        .to_string()
                } else {
                    " ".to_string()
                }
            })
            .collect::<Vec<String>>();

        if let Some(ref h) = self.highlight {
            if let Some(p) = h.position {
                let mut style = Style::default();

                if let Some(f) = h.fg {
                    style = style.fg(ansi_term::Colour::RGB(f.red, f.green, f.blue));
                }

                if let Some(b) = h.bg {
                    style = style.on(ansi_term::Colour::RGB(b.red, b.green, b.blue));
                }

                graph[p] = style.paint(graph[p].clone()).to_string();
            }
        }

        graph.into_iter().collect::<String>()
    }

    pub fn sparkfont(&self) -> String {
        let mut min = ::std::f64::MAX;
        let mut max = 0.;

        self.values.iter().filter_map(|i| *i).for_each(|i| {
            if i > max {
                max = i;
            }
            if i < min {
                min = i;
            }
        });

        let ratio = if min == max {
            1.
        } else {
            f64::from(self.font.size()) / (max - min)
        };

        let mut graph = self.values
            .iter()
            .map(|n| n.unwrap_or_else(|| 0.))
            .map(|n| (n - min) * ratio)
            .map(|n| format!("{},", n.floor()))
            .collect::<Vec<String>>();

        graph[0] = format!("{{{}", graph[0].clone());

        let last = graph.len() - 1;
        graph[last] = graph[last].replace(",", "}");

        if let Some(ref h) = self.highlight {
            if let Some(p) = h.position {
                let bg = match h.bg {
                    Some(b) => format!("background='{}'", b.hex()),
                    None => String::new(),
                };
                let fg = match h.fg {
                    Some(f) => format!("foreground='{}'", f.hex()),
                    None => String::new(),
                };
                graph[p] = format!("<span {} {}>{}</span>", bg, fg, graph[p].clone());
            }
        }

        format!(
            "<span font_desc='{}'>{}</span>",
            self.font,
            graph.into_iter().collect::<String>(),
        )
    }
}
