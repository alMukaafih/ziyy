use crate::parser::color::Color;

use super::{DuoEffect, Effect};

#[derive(Default)]
pub struct AnsiOptions {
    pub brightness: DuoEffect,
    pub under: DuoEffect,
    pub blink: Effect,
    pub hidden: Effect,
    pub italics: Effect,
    pub negetive: Effect,
    pub strike: Effect,
    pub fg_color: Color,
    pub bg_color: Color,
}
