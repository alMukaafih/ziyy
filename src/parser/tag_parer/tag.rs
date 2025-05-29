use std::{
    fmt::{Debug, Display, Write},
    io::Write as _,
    ops::Not,
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TagType {
    #[default]
    Open,
    Close,
    SelfClose,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Tag {
    pub r#type: TagType,
    style: u32,
    data: Vec<String>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            r#type: TagType::SelfClose,
            style: 0,
            data: vec![String::new(); 6],
        }
    }
}

impl Tag {
    pub fn inherit(&mut self, src: &Tag) {
        macro_rules! inherit_prop {
            (1, $f:tt, $g:tt ) => {
                if src.$f() && !self.$f() {
                    self.$g(src.$f());
                }
            };

            (2, $f:tt, $g:tt ) => {
                if !src.$f().is_empty() && self.$f().is_empty() {
                    self.$g(src.$f().to_string());
                }
            };
        }

        inherit_prop!(1, is_bold_set, set_bold);
        inherit_prop!(1, is_dim_set, set_dim);
        inherit_prop!(1, is_blink_set, set_blink);
        inherit_prop!(1, is_hidden_set, set_hidden);
        inherit_prop!(1, is_strike_set, set_strike);
        inherit_prop!(1, is_italics_set, set_italics);
        inherit_prop!(1, is_negative_set, set_negative);
        inherit_prop!(1, is_under_set, set_under);
        inherit_prop!(1, is_double_under_set, set_double_under);
        inherit_prop!(2, fg_color, set_fg_color);
        inherit_prop!(2, bg_color, set_bg_color);
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
        if n == 1 {
            true
        } else {
            false
        }
    }};
}

macro_rules! impl_tag {
    ( $( ( $i:expr, $set_x:tt, $x:tt, $is_set:tt ) ),*; $( ( $j:expr, $set_y:tt, $y:tt ) ),* ) => {
        impl Tag {
            const L: u32 = 31;

        $(
            pub fn $set_x(&mut self, value: bool) {
                set_style!(self.style, ($i * 2) + 1, true);
                set_style!(self.style, $i * 2, value);
            }

            pub fn $x(&self) -> bool {
                get_style!(self.style, $i * 2)
            }

            pub fn $is_set(&self) -> bool {
                get_style!(self.style, ($i * 2) + 1)
            }
        )*

        $(
            pub fn $set_y(&mut self, value: String) {
                self.data[$j] = value;
            }

            pub fn $y(&self) -> &String {
                &self.data[$j]
            }
        )*
        }
    };
}

impl_tag![
    (0, set_bold, bold, is_bold_set),
    (1, set_dim, dim, is_dim_set),
    (2, set_blink, blink, is_blink_set),
    (3, set_hidden, hidden, is_hidden_set),
    (4, set_strike, strike, is_strike_set),
    (5, set_italics, italics, is_italics_set),
    (6, set_negative, negative, is_negative_set),
    (7, set_under, under, is_under_set),
    (8, set_double_under, double_under, is_double_under_set);

    (0, set_name, name),
    (1, set_fg_color, fg_color),
    (2, set_bg_color, bg_color),
    (3, set_custom, custom),
    (4, set_src, src)
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

        tag.style &= 0x5555_5555;

        tag
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            macro_rules! write_prop {
                ( $f:tt, $g:tt, $prop:expr ) => {
                    if self.$f() {
                        f.write_fmt(format_args!(" {}=\"true\"", $prop))?;
                    } else if self.$g() {
                        f.write_fmt(format_args!(" {}=\"false\"", $prop))?;
                    }
                };
            }

            if self.name() == "$ansi" {
                return f.write_fmt(format_args!("$ansi: \"\\u{{1b}}[{}m\"", self.custom()));
            }

            match self.r#type {
                TagType::Open | TagType::SelfClose => f.write_str("<"),
                TagType::Close => f.write_str("</"),
            }?;

            f.write_str(self.name())?;

            if self.r#type != TagType::Close {
                write_prop!(dim, is_dim_set, "d");
                write_prop!(bold, is_bold_set, "b");
                write_prop!(italics, is_italics_set, "i");
                write_prop!(under, is_under_set, "u");
                write_prop!(blink, is_blink_set, "k");
                write_prop!(negative, is_negative_set, "n");
                write_prop!(hidden, is_hidden_set, "h");
                write_prop!(strike, is_strike_set, "s");
                write_prop!(double_under, is_double_under_set, "uu");
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
            return f.write_fmt(format_args!(
                "{}",
                " ".repeat(self.custom().parse::<usize>().unwrap_or(0))
            ));
        }

        let mut buf = vec![];
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop {
            ( $f:tt, $g:tt, $on:expr, $off:expr ) => {
                if self.$f() {
                    let _ = buf.write($on);
                } else if self.$g() {
                    let _ = buf.write($off);
                }
            };
        }

        write_prop!(dim, is_dim_set, b"2;", b"22;");
        write_prop!(bold, is_bold_set, b"1;", b"22;");
        write_prop!(italics, is_italics_set, b"3;", b"23;");
        write_prop!(under, is_under_set, b"4;", b"24;");
        write_prop!(blink, is_blink_set, b"5;", b"25;");
        write_prop!(negative, is_negative_set, b"7;", b"27;");
        write_prop!(hidden, is_hidden_set, b"8;", b"28;");
        write_prop!(strike, is_strike_set, b"9;", b"29;");
        write_prop!(double_under, is_double_under_set, b"21;", b"24;");

        if matches!(self.r#type, TagType::Close) {
            if !self.fg_color().is_empty() {
                let _ = buf.write(b"39;");
            }
            if !self.bg_color().is_empty() {
                let _ = buf.write(b"49;");
            }
        } else {
            let _ = buf.write(self.data[1].as_bytes());
            let _ = buf.write(self.data[2].as_bytes());
        }

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        if buf.len() == 3 {
            return Ok(());
        }

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}
