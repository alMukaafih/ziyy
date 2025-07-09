use crate::parser::color::Color;

use super::State;

#[derive(Default, Debug, PartialEq)]
pub enum Effect {
    #[default]
    None,
    Apply,
    Clear,
}

impl Effect {
    pub fn is_some(&self) -> bool {
        !matches!(self, Effect::None)
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

#[derive(Default)]
pub struct AnsiOptions {
    pub brightness: State,
    pub under: State,
    pub blink: Effect,
    pub hidden: Effect,
    pub italics: Effect,
    pub negetive: Effect,
    pub strike: Effect,
    pub fg_color: Color,
    pub bg_color: Color,
}
