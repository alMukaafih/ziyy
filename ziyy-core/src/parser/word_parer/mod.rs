use super::chunk::{Chunk, ChunkData};
use super::color::{Ansi256, Color, Rgb};
use crate::error::Error;
use crate::scanner::GenericScanner;
use crate::splitter::fragment::Fragment;
use ansi::Ansi;
use scanner::Scanner;
use std::collections::VecDeque;
use token::TokenType::*;
pub mod ansi;
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

impl Default for WordParser {
    fn default() -> Self {
        Self::new()
    }
}

impl WordParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, source: Fragment) -> Result<Vec<Chunk>, Error> {
        let mut span = source.span;
        let mut scanner = Scanner::new(source);
        let mut tokens: VecDeque<_> = scanner.scan_tokens().into();

        let mut next = || tokens.pop_front().unwrap_or_default();

        let mut chars = vec![];

        loop {
            let token = next();
            let c = match token.r#type {
                ESCAPE => '\x1B',
                ESCAPE_A => '\x07',
                ESCAPE_B => '\x08',
                ESCAPE_E => '\x1B',
                ESCAPE_F => '\x0C',
                ESCAPE_N => '\x0A',
                ESCAPE_R => '\x0D',
                ESCAPE_T => '\t',
                ESCAPE_V => '\x0B',
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
        span.tie_start();

        macro_rules! check_nl {
            () => {
                if chars[i] == '\n' {
                    span += (1, 0);
                }
            };
        }

        loop {
            if i >= len {
                break;
            }

            check_nl!();
            let c = chars[i];

            if c == '\x1B' && chars[i + 1] == '[' {
                i += 2;
                span += (0, 2);
                // Handle escape
                let j = i;

                if !matches!(chars[i], '\x30'..='\x39' | '\x3B' | '\x40'..='\x7E') {
                    while i < len && chars[i] != '\x1B' {
                        check_nl!();
                        i += 1;
                        span += (0, 1);
                    }
                    let word = chars[j..i].to_string();
                    chunks.push(Chunk {
                        data: ChunkData::Word(word),
                        span,
                    });
                    span.tie_end();
                    break;
                }

                while i < len && !matches!(chars[i], '\x40'..='\x7E') {
                    check_nl!();
                    i += 1;
                    span += (0, 1);
                }

                if chars[i] == 'm' {
                    // Handle escape sequence
                    let escape_sequence = chars[j..i].to_string();

                    if let Ok(ansi) = self.to_ansi(escape_sequence) {
                        chunks.push(Chunk {
                            data: ChunkData::Ansi(ansi),
                            span,
                        });
                        span.tie_end();
                    }
                } else {
                    while i < len && chars[i] != '\x1B' {
                        check_nl!();
                        i += 1;
                        span += (0, 1);
                    }
                    let word = chars[j..i].to_string();
                    chunks.push(Chunk {
                        data: ChunkData::Word(word),
                        span,
                    });
                    span.tie_end();
                }
                i += 1;
                span += (0, 1);

                continue;
            } else {
                let j = i;
                while i < len && chars[i] != '\x1B' {
                    check_nl!();
                    i += 1;
                    span += (0, 1);
                }
                let word = chars[j..i].to_string();
                chunks.push(Chunk {
                    data: ChunkData::Word(word),
                    span,
                });
                span.tie_end();
            }
            // Handle normal character
            i += 1;
            span += (0, 1);
        }

        Ok(chunks)
    }

    fn to_ansi(&self, source: String) -> Result<Ansi, i8> {
        // Convert ANSI escape codes to tags
        // This is a placeholder implementation
        let mut parts = source
            .split(';')
            .map(|x| {
                if x.is_empty() {
                    Ok(0)
                } else {
                    x.parse::<i32>().map_err(|_| 0)
                }
            })
            .collect::<VecDeque<_>>();

        let mut next = || parts.pop_front().unwrap_or(Err(-1));

        let mut ansi = Ansi::default();
        loop {
            let num = next();

            let num = match num {
                Ok(n) => n,
                Err(-1) => break,
                Err(_) => return Err(0),
            };

            match num {
                -1 => break,
                0 => ansi = Ansi::default(),

                1 => {
                    ansi.set_bold(true);
                    ansi.set_dim(false);
                    ansi.set_clear_bold(false);
                }
                2 => {
                    ansi.set_bold(false);
                    ansi.set_dim(true);
                    ansi.set_clear_bold(false);
                }
                22 => {
                    ansi.set_bold(false);
                    ansi.set_dim(false);
                    ansi.set_clear_bold(true);
                }

                3 => {
                    ansi.set_italics(true);
                    ansi.set_clear_italics(false);
                }
                23 => {
                    ansi.set_italics(false);
                    ansi.set_clear_italics(true);
                }

                4 => {
                    ansi.set_under(true);
                    ansi.set_double_under(false);
                    ansi.set_clear_under(false);
                }
                21 => {
                    ansi.set_under(false);
                    ansi.set_double_under(true);
                    ansi.set_clear_under(false);
                }
                24 => {
                    ansi.set_under(false);
                    ansi.set_double_under(false);
                    ansi.set_clear_under(true);
                }

                5 => {
                    ansi.set_blink(true);
                    ansi.set_clear_blink(false);
                }
                25 => {
                    ansi.set_blink(false);
                    ansi.set_clear_blink(true);
                }

                7 => {
                    ansi.set_negative(true);
                    ansi.set_clear_negative(false);
                }
                27 => {
                    ansi.set_negative(false);
                    ansi.set_clear_negative(true);
                }

                8 => {
                    ansi.set_hidden(true);
                    ansi.set_clear_hidden(false);
                }
                28 => {
                    ansi.set_hidden(false);
                    ansi.set_clear_hidden(true);
                }

                9 => {
                    ansi.set_strike(true);
                    ansi.set_clear_strike(false);
                }
                29 => {
                    ansi.set_strike(false);
                    ansi.set_clear_strike(true);
                }

                fg @ 30..=37 | fg @ 39 => ansi.set_fg_color(Color::four_bit(shrink!(fg))),
                bg @ 40..=47 | bg @ 49 => ansi.set_bg_color(Color::four_bit(shrink!(bg))),

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
                        ansi.set_fg_color(Color::Rgb(Rgb(38, shrink!(r), shrink!(g), shrink!(b))));
                    }

                    if num == 5 {
                        let fixed = next()?;
                        ansi.set_fg_color(Color::Ansi256(Ansi256(38, shrink!(fixed))));
                    }
                }

                48 => {
                    let num = next()?;
                    if num == 2 {
                        let r = next()?;
                        let g = next()?;
                        let b = next()?;
                        ansi.set_fg_color(Color::Rgb(Rgb(48, shrink!(r), shrink!(g), shrink!(b))));
                    }
                    if num == 5 {
                        let fixed = next()?;
                        ansi.set_fg_color(Color::Ansi256(Ansi256(48, shrink!(fixed))));
                    }
                }
                _ => {}
            }
        }

        Ok(ansi)
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
