use std::fmt::{Display, Write};
use std::io::Write as _;

use crate::parser::color::Color;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ansi {
    style: u32,
    colors: Vec<Color>,
}

impl Default for Ansi {
    fn default() -> Self {
        Self::new()
    }
}

impl Ansi {
    pub fn new() -> Self {
        Ansi {
            style: 0,
            colors: vec![Color::new(); 2],
        }
    }
}

macro_rules! set_style {
    ( $style:expr, $offset:expr, $value:expr ) => {
        if $value {
            $style |= 1 << (Ansi::L - $offset);
        } else {
            $style &= 0 << (Ansi::L - $offset);
        }
    };
}

macro_rules! get_style {
    ( $style:expr, $offset:expr ) => {{
        let n = ($style >> (Ansi::L - $offset)) & 1;
        if n == 1 { true } else { false }
    }};
}

macro_rules! impl_ansi {
    ( $( ( $i:expr, $set_x:tt, $x:tt ) ),*; $( ( $k:expr, $set_z:tt, $z:tt ) ),* ) => {
        impl Ansi {
            const L: u32 = 31;

        $(
            pub fn $set_x(&mut self, value: bool) {
                set_style!(self.style, $i, value);
            }

            pub fn $x(&self) -> bool {
                get_style!(self.style, $i)
            }
        )*

        $(
            pub fn $set_z(&mut self, value: Color) {
                self.colors[$k] = value;
            }

            pub fn $z(&self) -> &Color {
                &self.colors[$k]
            }
        )*
        }
    };
}

impl_ansi![
    (0, set_bold, bold),
    (1, set_dim, dim),
    (2, set_blink, blink),
    (3, set_hidden, hidden),
    (4, set_strike, strike),
    (5, set_italics, italics),
    (6, set_negative, negative),
    (7, set_under, under),
    (8, set_double_under, double_under),

    (15, set_clear_bold, clear_bold),
    (17, set_clear_blink, clear_blink),
    (18, set_clear_hidden, clear_hidden),
    (19, set_clear_strike, clear_strike),
    (20, set_clear_italics, clear_italics),
    (21, set_clear_negative, clear_negative),
    (22, set_clear_under, clear_under);

    (0, set_fg_color, fg_color),
    (1, set_bg_color, bg_color)
];

impl Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::with_capacity(128);
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop {
            ( $f:tt, $e:expr ) => {
                if self.$f() {
                    let _ = buf.write($e);
                }
            };
        }

        write_prop!(dim, b"2;");
        write_prop!(bold, b"1;");
        write_prop!(clear_bold, b"22;");

        write_prop!(italics, b"3;");
        write_prop!(clear_italics, b"23;");

        write_prop!(under, b"4;");
        write_prop!(double_under, b"21;");
        write_prop!(clear_under, b"24;");

        write_prop!(blink, b"5;");
        write_prop!(clear_blink, b"25;");

        write_prop!(negative, b"7;");
        write_prop!(clear_negative, b"27;");

        write_prop!(hidden, b"8;");
        write_prop!(clear_hidden, b"28;");

        write_prop!(strike, b"9;");
        write_prop!(clear_strike, b"29;");

        let _ = buf.write(self.fg_color().to_string().as_bytes());
        let _ = buf.write(self.bg_color().to_string().as_bytes());

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        if buf.len() == 3 {
            buf.clear();
        }

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}
