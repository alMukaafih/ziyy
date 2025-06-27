use super::chunk::{Chunk, ChunkData};
use super::color::{Ansi256, Color, Rgb};
use super::tag_parer::tag::{Tag, TagType};
use crate::common::Span;
use crate::error::Error;
use crate::scanner::GenericScanner;
use crate::splitter::fragment::Fragment;
use ansi::State;
use scanner::Scanner;
use token::Token;
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

#[doc(hidden)]
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
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        let mut chunks = vec![];

        let mut i = 0;
        let len = tokens.len();

        loop {
            if i >= len {
                break;
            }

            let c = tokens[i].literal;

            if c == '\x1b' && tokens[i + 1].literal == '[' {
                let g = i;
                i += 2;
                // Handle escape
                let h = i;

                if !matches!(tokens[i].literal, '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e') {
                    while i < len && tokens[i].literal != '\x1b' {
                        i += 1;
                    }
                    let word = tokens[g..i].to_string();
                    chunks.push(Chunk {
                        data: ChunkData::Word(word),
                        span: tokens[g..i].to_span(),
                    });

                    break;
                }

                while i < len && !matches!(tokens[i].literal, '\x40'..='\x7e') {
                    i += 1;
                }

                if tokens[i].literal == 'm' {
                    // Handle escape sequence
                    let escape_sequence = tokens[h..i].to_string();

                    if let Ok(tag) = self.ansi_to_tag(escape_sequence) {
                        chunks.push(Chunk {
                            data: ChunkData::Tag(tag),
                            span: tokens[g..i].to_span(),
                        });
                    }
                } else {
                    while i < len && tokens[i].literal != '\x1b' {
                        i += 1;
                    }
                    let word = tokens[h..i].to_string();
                    chunks.push(Chunk {
                        data: ChunkData::Word(word),
                        span: tokens[h..i].to_span(),
                    });
                }
                i += 1;

                continue;
            } else {
                let h = i;
                while i < len && tokens[i].literal != '\x1b' {
                    i += 1;
                }
                let word = tokens[h..i].to_string();
                chunks.push(Chunk {
                    data: ChunkData::Word(word),
                    span: tokens[h..i].to_span(),
                })
            }
            // Handle normal character
            // i += 1
        }

        Ok(chunks)
    }

    fn ansi_to_tag(&self, source: String) -> Result<Tag, i8> {
        // Convert ANSI escape codes to tags
        let parts = source.split(';');

        let mut segments = Vec::with_capacity(10);
        for part in parts {
            if part.is_empty() {
                segments.push(0);
            } else {
                segments.push(part.parse::<i32>().map_err(|_| 0)?);
            }
        }

        let mut parts = segments.iter().peekable();

        let mut tag = Tag::default();
        loop {
            let num = parts.next();

            let num = match num {
                Some(n) => *n,
                None => break,
            };

            match num {
                -1 => break,
                0 => tag.reset_styles(),

                1 => {
                    tag.set_brightness(State::A);
                }
                2 => {
                    tag.set_brightness(State::B);
                }
                22 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_brightness(match num {
                            1 => State::BA,
                            2 => State::AB,
                            _ => State::E,
                        });
                        parts.next();
                    } else {
                        tag.set_brightness(State::E);
                    }
                }

                3 => {
                    tag.set_italics(true);
                }
                23 => {
                    tag.set_italics(false);
                }

                4 => {
                    tag.set_under(State::A);
                }
                21 => {
                    tag.set_under(State::B);
                }
                24 => {
                    let num = parts.peek();
                    if let Some(num) = num {
                        tag.set_under(match num {
                            4 => State::BA,
                            21 => State::AB,
                            _ => State::E,
                        });
                        parts.next();
                    } else {
                        tag.set_under(State::E);
                    }
                }

                5 => {
                    tag.set_blink(true);
                }
                25 => {
                    tag.set_blink(false);
                }

                7 => {
                    tag.set_negative(true);
                }
                27 => {
                    tag.set_negative(false);
                }

                8 => {
                    tag.set_hidden(true);
                }
                28 => {
                    tag.set_hidden(false);
                }

                9 => {
                    tag.set_strike(true);
                }
                29 => {
                    tag.set_strike(false);
                }

                fg @ 30..=37 | fg @ 39 => tag.set_fg_color(Color::four_bit(shrink!(fg))),
                bg @ 40..=47 | bg @ 49 => tag.set_bg_color(Color::four_bit(shrink!(bg))),

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
                    let num = *parts.next().ok_or(0)?;
                    if num == 2 {
                        let r = *parts.next().ok_or(0)?;
                        let g = *parts.next().ok_or(0)?;
                        let b = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(38, shrink!(r), shrink!(g), shrink!(b))));
                    }

                    if num == 5 {
                        let fixed = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Ansi256(Ansi256(38, shrink!(fixed))));
                    }
                }

                48 => {
                    let num = *parts.next().ok_or(0)?;
                    if num == 2 {
                        let r = *parts.next().ok_or(0)?;
                        let g = *parts.next().ok_or(0)?;
                        let b = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Rgb(Rgb(48, shrink!(r), shrink!(g), shrink!(b))));
                    }
                    if num == 5 {
                        let fixed = *parts.next().ok_or(0)?;
                        tag.set_fg_color(Color::Ansi256(Ansi256(48, shrink!(fixed))));
                    }
                }
                _ => {}
            }
        }

        tag.set_name("$ansi".to_string());
        tag.r#type = TagType::Open;

        Ok(tag)
    }
}

trait Transform {
    fn to_string(&self) -> String;
    fn to_span(&self) -> Span;
}

impl Transform for [Token] {
    fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len());

        for token in self {
            text.push(token.literal)
        }

        text
    }

    fn to_span(&self) -> Span {
        let mut span = Span::inserted();
        for token in self {
            if span == Span::inserted() {
                span = token.span;
            } else {
                span += token.span;
            }
        }

        span
    }
}
