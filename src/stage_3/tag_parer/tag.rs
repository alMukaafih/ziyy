use std::{
    fmt::{Display, Write},
    io::Write as _,
    ops::Not,
};

#[derive(Default, Clone, Debug)]
pub enum TagType {
    #[default]
    Open,
    Close,
    SelfClose,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub r#type: TagType,
    style: u16,
    data: Vec<String>,
}

impl Default for Tag {
    fn default() -> Self {
        Self {
            r#type: TagType::SelfClose,
            style: 0,
            data: vec!["".to_string(); 4],
        }
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
    ( $( ( $i:expr, $set_x:tt, $x:tt ) ),* ) => {
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
        }
    };
}

macro_rules! impl_tag2 {
    ( $( ( $i:expr, $set_x:tt, $x:tt ) ),* ) => {
        impl Tag {
        $(
            pub fn $set_x(&mut self, value: String) {
                self.data[$i] = value;
            }

            pub fn $x(&self) -> &String {
                &self.data[$i]
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
    (8, set_double_under, double_under)
];

impl_tag2![
    (0, set_name, name),
    (1, set_fg_color, fg_color),
    (2, set_bg_color, bg_color),
    (3, set_custom, custom)
];

impl Not for Tag {
    type Output = Self;

    fn not(self) -> Self::Output {
        let mut tag = self;

        match tag.r#type {
            TagType::Open => tag.r#type = TagType::Close,
            TagType::Close => tag.r#type = TagType::Open,
            TagType::SelfClose => tag.r#type = TagType::SelfClose,
        }

        tag.data[1] = String::new();
        tag.data[2] = String::new();

        tag
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.data[0] == "let" {
            return Ok(());
        }
        let mut buf = vec![];
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop {
            ( $f:tt, $on:expr, $off:expr ) => {
                match self.r#type {
                    TagType::Open => {
                        if self.$f() {
                            let _ = buf.write($on);
                        }
                    }
                    TagType::Close => {
                        if self.$f() {
                            let _ = buf.write($off);
                        }
                    }
                    TagType::SelfClose => {}
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

        let _ = buf.write(self.data[1].as_bytes());
        let _ = buf.write(self.data[2].as_bytes());

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
