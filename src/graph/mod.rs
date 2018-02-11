mod font;

use ansi_term;

pub use self::font::{Font, Weight};
use self::font::SPARKS_FONT_SIZE;
pub use self::font::Style;
use color::Color;

#[derive(Copy, Clone, Debug, Default, Deserialize)]
pub struct Highlight {
    pub position: Option<usize>,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
}

#[derive(Debug, Default)]
pub struct Graph {
    values: Vec<Option<f64>>,
    pub font: Font,
    pub highlight: Option<Highlight>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
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
        self.highlight = Some(if highlight.position.is_none() {
            Highlight {
                position: *position,
                ..*highlight
            }
        } else {
            *highlight
        });
        self
    }

    pub fn font_style(&mut self, font_style: &Style) -> &mut Self {
        self.font.style = *font_style;
        self
    }

    pub fn font_weight(&mut self, font_weight: &Weight) -> &mut Self {
        self.font.weight = *font_weight;
        self
    }

    // Giving credit where credit is due: this was heavily inspired by Jiři Šebele's work:
    // https://github.com/jiri/rust-spark.
    pub fn sparkline(&self) -> String {
        use ansi_term::Style;
        let bars = "▁▂▃▄▅▆▇█";

        let (min, _, ratio) = calculate_min_max_and_ratio(&self.values, bars.chars().count() - 1);

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

    // Giving credit where credit is due: this was heavily inspired by Jiři Šebele's work:
    // https://github.com/jiri/rust-spark.
    pub fn sparkfont(&self) -> String {
        let (min, _, ratio) = calculate_min_max_and_ratio(&self.values, SPARKS_FONT_SIZE);

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

fn calculate_min_max_and_ratio(values: &[Option<f64>], size: usize) -> (f64, f64, f64) {
    let mut min = ::std::f64::MAX;
    let mut max = ::std::f64::MIN;

    values.iter().filter_map(|i| *i).for_each(|i| {
        if i > max {
            max = i;
        }

        if i > min {
            min = i;
        }
    });

    // Compare if max and min are equal, as suggested by clippy
    let ratio = if (max - min).abs() < ::std::f64::EPSILON {
        1.
    } else {
        size as f64 / (max - min)
    };

    (min, max, ratio)
}
