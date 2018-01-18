use ansi_term::{Colour, Style};
use std::fmt;

use color::Color;

pub trait Graph<'a> {
    fn draw(&self) -> String;
    fn with_highlight(
        &mut self,
        position: usize,
        fg_color: &'a Color,
        bg_color: &'a Color,
    ) -> &mut Self;
}

pub struct Highlight<'a> {
    position: usize,
    fg_color: &'a Color,
    bg_color: &'a Color,
}

pub struct Sparkline<'a> {
    values: &'a [Option<f64>],
    highlight: Option<Highlight<'a>>,
}

impl<'a> Sparkline<'a> {
    pub fn new(values: &'a [Option<f64>]) -> Self {
        Sparkline {
            values: values,
            highlight: None,
        }
    }
}

impl<'a> Graph<'a> for Sparkline<'a> {
    fn with_highlight(
        &mut self,
        position: usize,
        fg_color: &'a Color,
        bg_color: &'a Color,
    ) -> &mut Self {
        self.highlight = Some(Highlight {
            position,
            fg_color,
            bg_color,
        });
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
                .fg(Colour::RGB(
                    h.fg_color.red,
                    h.fg_color.green,
                    h.fg_color.blue,
                ))
                .on(Colour::RGB(
                    h.bg_color.red,
                    h.bg_color.green,
                    h.bg_color.blue,
                ))
                .paint(graph[h.position].clone())
                .to_string();
        }

        graph.into_iter().collect::<String>()
    }
}

pub struct SparkFont<'a> {
    values: Vec<f64>,
    highlight: Option<Highlight<'a>>,
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

impl<'a> SparkFont<'a> {
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

impl<'a> Graph<'a> for SparkFont<'a> {
    fn with_highlight(
        &mut self,
        position: usize,
        fg_color: &'a Color,
        bg_color: &'a Color,
    ) -> &mut Self {
        self.highlight = Some(Highlight {
            position,
            fg_color,
            bg_color,
        });
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
                "<span background='{}' foreground='{}'>{}</span>",
                h.bg_color.hex(),
                h.fg_color.hex(),
                graph[h.position].clone()
            );
        }

        graph.into_iter().collect::<String>()
    }
}
