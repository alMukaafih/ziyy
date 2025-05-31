use std::{
    fmt::{Debug, Display, Write},
    io::Write as _,
    ops::Not,
};

use crate::parser::color::Color;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    #[default]
    Open,
    Close,
    SelfClose,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Value {
    Color(Color),
    String(String),
}

impl Value {
    fn color(&self) -> &Color {
        match self {
            Value::Color(color) => color,
            Value::String(_) => panic!("illegal"),
        }
    }

    fn string(&self) -> &String {
        match self {
            Value::Color(_) => panic!("illegal"),
            Value::String(string) => string,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub r#type: TagType,
    style: u16,
    data: Vec<Value>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            r#type: TagType::SelfClose,
            style: 0,
            data: vec![
                Value::String(String::new()),
                Value::Color(Color::new()),
                Value::Color(Color::new()),
                Value::String(String::new()),
                Value::String(String::new()),
            ],
        }
    }
}

impl Tag {
    pub fn with_name(name: &str) -> Self {
        let mut tag = Tag::default();
        tag.set_name(name.to_string());
        tag
    }

    pub fn inherit(&mut self, src: &Tag) {
        macro_rules! inherit_prop {
            (1, $f:tt, $g:tt ) => {
                if src.$f() && !self.$f() {
                    self.$g(src.$f());
                }
            };

            (2, $f:tt, $g:tt ) => {
                if !src.$f().is_empty() && self.$f().is_empty() {
                    self.$g(src.$f().clone());
                }
            };
        }

        inherit_prop!(1, bold, set_bold);
        inherit_prop!(1, dim, set_dim);
        inherit_prop!(1, blink, set_blink);
        inherit_prop!(1, hidden, set_hidden);
        inherit_prop!(1, strike, set_strike);
        inherit_prop!(1, italics, set_italics);
        inherit_prop!(1, negative, set_negative);
        inherit_prop!(1, under, set_under);
        inherit_prop!(1, double_under, set_double_under);
        inherit_prop!(2, fg_color, set_fg_color);
        inherit_prop!(2, bg_color, set_bg_color);
    }

    pub fn clear_styles(&mut self) {
        macro_rules! clear_prop {
            (1, $f:tt ) => {
                self.$f(false)
            };

            (2, $f:tt ) => {
                self.$f(Color::new())
            };
        }

        clear_prop!(1, set_bold);
        clear_prop!(1, set_dim);
        clear_prop!(1, set_blink);
        clear_prop!(1, set_hidden);
        clear_prop!(1, set_strike);
        clear_prop!(1, set_italics);
        clear_prop!(1, set_negative);
        clear_prop!(1, set_under);
        clear_prop!(1, set_double_under);
        clear_prop!(2, set_fg_color);
        clear_prop!(2, set_bg_color);
    }
}

macro_rules! set_style {
    ( $style:expr, $offset:expr, $value:expr ) => {
        if $value {
            $style |= 1 << (Tag::L - $offset);
        } else {
            $style &= 0 << (Tag::L - $offset);
        }
    };
}

macro_rules! get_style {
    ( $style:expr, $offset:expr ) => {{
        let n = ($style >> (Tag::L - $offset)) & 1;
        if n == 1 { true } else { false }
    }};
}

macro_rules! impl_tag {
    ( $( ( $i:expr, $set_x:tt, $x:tt ) ),*; $( ( $j:expr, $set_y:tt, $y:tt ) ),*; $( ( $k:expr, $set_z:tt, $z:tt ) ),* ) => {
        impl Tag {
            const L: u16 = 15;

        $(
            pub fn $set_x(&mut self, value: bool) {
                set_style!(self.style, $i, value);
            }

            pub fn $x(&self) -> bool {
                get_style!(self.style, $i)
            }
        )*

        $(
            pub fn $set_y(&mut self, value: String) {
                self.data[$j] = Value::String(value);
            }

            pub fn $y(&self) -> &String {
                &self.data[$j].string()
            }
        )*

        $(
            pub fn $set_z(&mut self, value: Color) {
                self.data[$k] = Value::Color(value);
            }

            pub fn $z(&self) -> &Color {
                &self.data[$k].color()
            }
        )*
        }
    };
}

impl_tag![
    (0, set_bold, bold),
    (1, set_dim, dim),
    (2, set_blink, blink),
    (3, set_hidden, hidden),
    (4, set_strike, strike),
    (5, set_italics, italics),
    (6, set_negative, negative),
    (7, set_under, under),
    (8, set_double_under, double_under);

    (0, set_name, name),
    (3, set_custom, custom),
    (4, set_src, src);

    (1, set_fg_color, fg_color),
    (2, set_bg_color, bg_color)
];

impl Not for Tag {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut tag = self;

        tag.r#type = match tag.r#type {
            TagType::Open => TagType::Close,
            TagType::Close => TagType::Open,
            TagType::SelfClose => TagType::SelfClose,
        };

        tag
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            macro_rules! write_prop {
                ( $f:tt, $prop:expr ) => {
                    if self.$f() {
                        match self.r#type {
                            TagType::Open => f.write_fmt(format_args!(" {}=\"true\"", $prop))?,
                            TagType::Close => f.write_fmt(format_args!(" {}=\"false\"", $prop))?,
                            TagType::SelfClose => {}
                        }
                    }
                };
            }

            match self.r#type {
                TagType::Open | TagType::SelfClose => f.write_str("<"),
                TagType::Close => f.write_str("</"),
            }?;

            f.write_str(self.name())?;

            if self.r#type != TagType::Close {
                write_prop!(dim, "d");
                write_prop!(bold, "b");
                write_prop!(italics, "i");
                write_prop!(under, "u");
                write_prop!(blink, "k");
                write_prop!(negative, "n");
                write_prop!(hidden, "h");
                write_prop!(strike, "s");
                write_prop!(double_under, "uu");
            }

            match self.r#type {
                TagType::Open | TagType::Close => f.write_str(">"),
                TagType::SelfClose => f.write_str("/>"),
            }?;

            return Ok(());
        }

        if self.name() == "br" {
            return f.write_str("\n");
        } else if self.name() == "p" && !self.custom().is_empty() {
            f.write_fmt(format_args!(
                "{}",
                " ".repeat(self.custom().parse::<usize>().unwrap_or(0))
            ))?;
        }

        let mut buf = Vec::with_capacity(128);
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop {
            ( $f:tt, $on:expr, $off:expr ) => {
                if self.$f() {
                    match self.r#type {
                        TagType::Open => {
                            let _ = buf.write($on);
                        }
                        TagType::Close => {
                            let _ = buf.write($off);
                        }
                        TagType::SelfClose => {}
                    }
                }
            };
        }

        write_prop!(dim, b"2;", b"22;");
        write_prop!(bold, b"1;", b"22;");
        write_prop!(italics, b"3;", b"23;");
        write_prop!(under, b"4;", b"24;");
        write_prop!(blink, b"5;", b"25;");
        write_prop!(negative, b"7;", b"27;");
        write_prop!(hidden, b"8;", b"28;");
        write_prop!(strike, b"9;", b"29;");
        write_prop!(double_under, b"21;", b"24;");

        if matches!(self.r#type, TagType::Close) {
            if !self.fg_color().is_empty() {
                let _ = buf.write(b"39;");
            }
            if !self.bg_color().is_empty() {
                let _ = buf.write(b"49;");
            }
        } else {
            let _ = buf.write(self.fg_color().to_string().as_bytes());
            let _ = buf.write(self.bg_color().to_string().as_bytes());
        }

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        if buf.len() == 3 {
            buf.clear();
        }

        if self.name() == "a" {
            match self.r#type {
                TagType::Open => {
                    let _ = buf.write(b"\x1b]8;;");
                    let _ = buf.write(self.custom().as_bytes());
                    let _ = buf.write(b"\x1b\\");
                }
                TagType::Close => {
                    let _ = buf.write(b"\x1b]8;;\x1b\\");
                }
                TagType::SelfClose => {}
            }
        }

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}
