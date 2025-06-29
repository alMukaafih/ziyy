use crate::parser::color::Color;

use super::State;

#[derive(Default, Debug, PartialEq)]
pub enum Style {
    #[default]
    None,
    Apply,
    Clear,
}

impl Style {
    pub fn is_some(&self) -> bool {
        !matches!(self, Style::None)
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

#[derive(Default)]
pub struct AnsiOptions {
    pub brightness: State,
    pub under: State,
    pub blink: Style,
    pub hidden: Style,
    pub italics: Style,
    pub negetive: Style,
    pub strike: Style,
    pub fg_color: Color,
    pub bg_color: Color,
}
