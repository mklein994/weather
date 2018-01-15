use ansi_term::{Colour, Style};
use std::fmt;

pub trait Graph {
    fn draw(&self) -> String;
    fn with_highlight(&mut self, position: usize, color: (u8, u8, u8, f64)) -> &mut Self;
}

pub struct Highlight {
    position: usize,
    color: (u8, u8, u8, f64),
}

pub struct Sparkline {
    values: Vec<Option<f64>>,
    highlight: Option<Highlight>,
}

impl Sparkline {
    pub fn new(values: &[Option<f64>]) -> Self {
        Sparkline {
            values: values.clone().to_vec(),
            highlight: None,
        }
    }
}

impl Graph for Sparkline {
    fn with_highlight(&mut self, position: usize, color: (u8, u8, u8, f64)) -> &mut Self {
        self.highlight = Some(Highlight { position, color });
        self
    }

    fn draw(&self) -> String {
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
            graph[h.position] = Style::default()
                .on(Colour::RGB(h.color.0, h.color.1, h.color.2))
                .paint(graph[h.position].clone())
                .to_string();
        }

        graph.into_iter().collect::<String>()
    }
}

pub struct SparkFont {
    values: Vec<f64>,
    highlight: Option<Highlight>,
    font: FontType,
}

pub enum FontType {
    BarMedium,
    BarNarrow,
    BarThin,
    DotMedium,
    DotSmall,
    DotlineMedium,
}

impl FontType {
    fn size(&self) -> u32 {
        match self {
            _ => 100,
        }
    }
}

impl fmt::Display for FontType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                _ => "made it",
            }
        )
    }
}

impl SparkFont {
    pub fn new(values: Vec<f64>) -> Self {
        SparkFont {
            values,
            highlight: None,
            font: FontType::BarMedium,
        }
    }

    pub fn font(&mut self, font: FontType) -> &mut Self {
        self.font = font;
        self
    }
}

impl Graph for SparkFont {
    fn with_highlight(&mut self, position: usize, color: (u8, u8, u8, f64)) -> &mut Self {
        self.highlight = Some(Highlight { position, color });
        self
    }

    fn draw(&self) -> String {
        let mut min = &::std::f64::MAX;
        let mut max = &0.;

        self.values.iter().for_each(|i| {
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
            (self.font.size()) as f64 / (max - min)
        };

        let mut graph = self.values
            .iter()
            .map(|n| (n - min) * ratio)
            .map(|n| format!("{},", n.floor()))
            .collect::<Vec<String>>();
        graph[0] = format!("{{{}", graph[0].clone());
        let last = graph.len() - 1;
        graph[last] = format!("{}}}", graph[last].clone());

        if let Some(ref h) = self.highlight {
            graph[h.position] = format!(
                "<span background='rgba({},{},{},{})'>{}</span>",
                h.color.0,
                h.color.1,
                h.color.2,
                h.color.3,
                graph[h.position].clone()
            );
        }

        graph.into_iter().collect::<String>()
    }
}
