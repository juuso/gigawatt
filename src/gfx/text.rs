use crate::gfx::{Rgb24, Srgb};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TermColor {
    Palette(u8),
    Rgb(Rgb24),
}

impl From<Srgb> for TermColor {
    fn from(s: Srgb) -> Self {
        TermColor::Rgb(s.into())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
struct Style {
    fg: Option<TermColor>,
    bg: Option<TermColor>,
    bold: bool,
}

fn encode_fg(color: &Option<TermColor>) -> String {
    match color {
        Some(TermColor::Palette(i)) if *i <= 7 => (30 + i).to_string(),
        Some(TermColor::Palette(i)) if *i <= 15 => (90 + i - 8).to_string(),
        Some(TermColor::Palette(i)) => format!("38;5;{}", i),
        Some(TermColor::Rgb((r, g, b))) => format!("38;2;{r};{g};{b}"),
        None => "39".into(),
    }
}

fn encode_bg(color: &Option<TermColor>) -> String {
    match color {
        Some(TermColor::Palette(i)) if *i <= 7 => (40 + i).to_string(),
        Some(TermColor::Palette(i)) if *i <= 15 => (100 + i - 8).to_string(),
        Some(TermColor::Palette(i)) => format!("48;5;{}", i),
        Some(TermColor::Rgb((r, g, b))) => format!("48;2;{r};{g};{b}"),
        None => "49".into(),
    }
}

fn encode_bold(on: bool) -> &'static str {
    if on { "1" } else { "22" }
}

fn wrap_sgr(sgr: &str, wrappers: Option<NonPrintingWrappers>) -> String {
    match wrappers {
        Some((start, end)) => format!("{start}{sgr}{end}"),
        None => sgr.into(),
    }
}

fn encode_style(
    from: Option<&Style>,
    to: &Style,
    wrappers: Option<NonPrintingWrappers>,
) -> Option<String> {
    let sgr = match from {
        Some(f) => encode_style_diff(f, to),
        None => Some(encode_style_full(to)),
    }?;

    Some(wrap_sgr(&sgr, wrappers))
}

fn encode_style_full(style: &Style) -> String {
    let mut parts = Vec::new();
    parts.push(encode_fg(&style.fg));
    parts.push(encode_bg(&style.bg));
    if style.bold {
        parts.push(encode_bold(style.bold).into());
    }

    format!("\x1b[{}m", parts.join(";"))
}

fn encode_style_diff(from: &Style, to: &Style) -> Option<String> {
    if from == to {
        return None;
    }

    let mut parts = Vec::new();

    if from.fg != to.fg {
        parts.push(encode_fg(&to.fg));
    }

    if from.bg != to.bg {
        parts.push(encode_bg(&to.bg));
    }

    if from.bold != to.bold {
        parts.push(encode_bold(to.bold).into());
    }

    Some(format!("\x1b[{}m", parts.join(";")))
}

pub fn bold(s: &str) -> String {
    format!("\x1b[1m{s}\x1b[0m")
}

pub fn green(s: &str) -> String {
    format!("\x1b[32m{s}\x1b[0m")
}

#[derive(Clone)]
struct Fragment {
    text: String,
    style: Style,
}

#[derive(Default)]
pub struct TextBuilder {
    style: Style,
    fragments: Vec<Fragment>,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fg(&mut self, color: Option<TermColor>) -> &mut Self {
        self.style.fg = color;
        self
    }

    pub fn bg(&mut self, color: Option<TermColor>) -> &mut Self {
        self.style.bg = color;
        self
    }

    pub fn bold(&mut self, on: bool) -> &mut Self {
        self.style.bold = on;
        self
    }

    pub fn text(&mut self, text: &str) -> &mut Self {
        self.fragments.push(Fragment {
            text: text.into(),
            style: self.style,
        });
        self
    }

    pub fn build(&self) -> StyledText {
        StyledText {
            fragments: self.fragments.clone(),
        }
    }
}

pub type NonPrintingWrappers = (&'static str, &'static str);

pub struct StyledText {
    fragments: Vec<Fragment>,
}

impl StyledText {
    pub fn render(&self, wrappers: Option<NonPrintingWrappers>) -> String {
        let mut result = String::new();

        if self.fragments.is_empty() {
            return result;
        }

        let mut style: Option<&Style> = None;

        for frag in &self.fragments {
            if let Some(s) = encode_style(style, &frag.style, wrappers) {
                result.push_str(&s);
            }
            result.push_str(&frag.text);
            style = Some(&frag.style);
        }

        let reset = wrap_sgr("\x1b[0m", wrappers);
        result.push_str(&reset);
        result
    }
}
