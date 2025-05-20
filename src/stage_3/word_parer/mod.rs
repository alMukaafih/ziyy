use super::chunk::Chunk;
use super::color::Color;
use super::tag_parer::tag::Tag;
use crate::error::Error;
use crate::scanner::GenericScanner;
use crate::stage_2::fragment::Fragment;
use scanner::Scanner;
use std::collections::VecDeque;
use token::TokenType::*;
mod scanner;
mod token;

macro_rules! shrink {
    ($num:expr) => {{
        if $num > 255 {
            255u8
        } else if $num < 0 {
            0u8
        } else {
            $num as u8
        }
    }};
}

pub struct WordParser;

impl WordParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, source: Fragment) -> Result<Vec<Chunk>, Error> {
        let mut scanner = Scanner::new(source);
        let mut tokens: VecDeque<_> = scanner.scan_tokens().into();

        let mut next = || tokens.pop_front().unwrap_or_default();

        let mut chars = vec![];

        loop {
            let token = next();
            let c = match token.r#type {
                ESCAPE => '\x1b',
                ESCAPE_A => '\x07',
                ESCAPE_B => '\x08',
                ESCAPE_E => '\x1b',
                ESCAPE_F => '\x0c',
                ESCAPE_N => '\x0a',
                ESCAPE_R => '\x0d',
                ESCAPE_T => '\t',
                ESCAPE_V => '\x0b',
                ESCAPE_0 => {
                    let num = u32::from_str_radix(&token.lexeme[2..], 8).unwrap();
                    char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER)
                }
                ESCAPE_X => {
                    let num = u32::from_str_radix(&token.lexeme[2..], 16).unwrap();
                    char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER)
                }
                ESCAPE_U => {
                    let num = u32::from_str_radix(&token.lexeme[2..], 16).unwrap();
                    char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER)
                }
                ESCAPE_LESS => '<',
                ESCAPE_GREATER => '>',
                ESCAPE_BACKSLASH => '\\',
                TEXT => {
                    let chs = token.lexeme.chars().collect::<Vec<_>>();
                    chars.extend(chs);
                    continue;
                }
                EOF => break,
            };
            chars.push(c);
        }

        let mut chunks = vec![];

        let mut i = 0;
        let len = chars.len();
        loop {
            if i >= len {
                break;
            }

            let c = chars[i];
            if c == '\x1b' && chars[i + 1] == '[' {
                i += 2;
                // Handle escape
                let j = i;

                if !matches!(chars[i], '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e') {
                    while i < len && chars[i] != '\x1b' {
                        i += 1;
                    }
                    let word = chars[j..i].to_string();
                    chunks.push(Chunk::Word(word));
                    break;
                }

                while i < len && !matches!(chars[i], '\x40'..='\x7e') {
                    i += 1;
                }

                if chars[i] == 'm' {
                    // Handle escape sequence
                    let escape_sequence = chars[j..i].to_string();

                    if let Ok(tag) = self.ansi_to_tag(escape_sequence) {
                        chunks.push(Chunk::new_tag(tag));
                    }
                } else {
                    while i < len && chars[i] != '\x1b' {
                        i += 1;
                    }
                    let word = chars[j..i].to_string();
                    chunks.push(Chunk::Word(word));
                }
                i += 1;

                continue;
            } else {
                let j = i;
                while i < len && chars[i] != '\x1b' {
                    i += 1;
                }
                let word = chars[j..i].to_string();
                chunks.push(Chunk::Word(word));
            }
            // Handle normal character
            i += 1;
        }

        Ok(chunks)
    }

    fn ansi_to_tag(&self, ansi: String) -> Result<Tag, i8> {
        // Convert ANSI escape codes to tags
        // This is a placeholder implementation
        let mut parts = ansi
            .split(';')
            .map(|x| i32::from_str_radix(x, 10).map_err(|_| 0))
            .collect::<VecDeque<_>>();

        let mut next = || parts.pop_front().unwrap_or(Err(-1));

        let mut tag = Tag::default();
        loop {
            let num = next();

            let num = match num {
                Ok(n) => n,
                Err(-1) => break,
                Err(_) => return Err(0),
            };

            match num {
                -1 => break,
                0 => tag = Tag::default(),

                1 => tag.set_bold(true),
                2 => tag.set_dim(true),
                22 => {
                    tag.set_bold(false);
                    tag.set_dim(false);
                }

                3 => tag.set_italics(true),
                23 => tag.set_italics(false),

                4 => tag.set_under(true),
                21 => tag.set_double_under(true),
                24 => {
                    tag.set_under(false);
                    tag.set_double_under(false);
                }

                5 => tag.set_blink(true),
                25 => tag.set_blink(false),

                7 => tag.set_negative(true),
                27 => tag.set_negative(false),

                8 => tag.set_hidden(true),
                28 => tag.set_hidden(true),

                9 => tag.set_strike(true),
                29 => tag.set_strike(false),

                fg @ 30..=37 | fg @ 39 => tag.set_fg_color(Color::four_bit(shrink!(fg)).into()),
                bg @ 40..=47 | bg @ 49 => tag.set_bg_color(Color::four_bit(shrink!(bg)).into()),

                /* 90 => tag.fg_color = "black".into(),
                91 => tag.fg_color = "red".into(),
                92 => tag.fg_color = "green".into(),
                93 => tag.fg_color = "yellow".into(),
                94 => tag.fg_color = "blue".into(),
                95 => tag.fg_color = "magenta".into(),
                96 => tag.fg_color = "cyan".into(),
                97 => tag.fg_color = "white".into(),

                100 => tag.bg_color = "black".into(),
                101 => tag.bg_color = "red".into(),
                102 => tag.bg_color = "green".into(),
                103 => tag.bg_color = "yellow".into(),
                104 => tag.bg_color = "blue".into(),
                105 => tag.bg_color = "magenta".into(),
                106 => tag.bg_color = "cyan".into(),
                107 => tag.bg_color = "white".into(), */
                38 => {
                    let num = next()?;
                    if num == 2 {
                        let r = next()?;
                        let g = next()?;
                        let b = next()?;
                        tag.set_fg_color(format!(
                            "38;2;{};{};{};",
                            shrink!(r),
                            shrink!(g),
                            shrink!(b)
                        ));
                    }

                    if num == 5 {
                        let byte = next()?;
                        tag.set_fg_color(format!("38;5;{};", shrink!(byte)));
                    }
                }

                48 => {
                    let num = next()?;
                    if num == 2 {
                        let r = next()?;
                        let g = next()?;
                        let b = next()?;
                        tag.set_bg_color(format!(
                            "48;2;{};{};{};",
                            shrink!(r),
                            shrink!(g),
                            shrink!(b)
                        ));
                    }
                    if num == 5 {
                        let byte = next()?;
                        tag.set_bg_color(format!("48;5;{};", shrink!(byte)));
                    }
                }
                _ => {}
            }
        }

        Ok(tag)
    }
}

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len());

        for ch in self {
            text.push(*ch)
        }

        text
    }
}
