use std::io::Write;

use crate::{
    assign_prop_bool, assign_prop_color, assign_prop_cond, assign_prop_value,
    color::{channel::Channel, Color},
    error::ErrorKind,
    get_num,
    num::str_to_u32,
    own,
    scanner::token::TokenKind,
    style::{Condition, Style},
    Error,
};

use super::{
    expect_token,
    tag::{Tag, TagName, TagType, Value},
    Parser,
};

/* const KEYWORDS: &[&str] = &[
    "a", "b", "blink", "br", "c", "dim", "e", "h", "i", "invert", "let", "p", "u", "uu", "x",
    "ziyy",
]; */

#[derive(Clone)]
pub enum Chunk {
    Tag(Tag),
    Text(String),
    WhiteSpace(String),
    Eof,
}

impl<T: AsRef<str>> Parser<T> {
    pub(super) fn parse_chunk(&mut self) -> Result<Chunk, Error> {
        if let Some(chunk) = self.next_chunk.clone() {
            self.next_chunk = None;
            return Ok(chunk);
        }

        let mut token;
        let r#type = loop {
            token = self.scanner.scan_token()?;
            match token.kind {
                TokenKind::Less => break TagType::Open,
                TokenKind::LessSlash => break TagType::Close,

                TokenKind::Text => {
                    //handle_text!(self, token.content);
                    return Ok(Chunk::Text(own!(token.content)));
                }

                TokenKind::WhiteSpace => {
                    //handle_white_space!(self, token.content)
                    return Ok(Chunk::WhiteSpace(own!(token.content)));
                }

                TokenKind::Eof => {
                    return Ok(Chunk::Eof);
                }

                TokenKind::Esc0 => {
                    let oct = get_num!(str_to_u32(&token.content[2..], 8), &token) as i8 as u8;

                    self.buf.push(oct);
                    continue;
                }

                TokenKind::EscX => {
                    let hex = get_num!(str_to_u32(&token.content[2..], 16), &token) as i8 as u8;
                    self.buf.push(hex);
                    continue;
                }

                TokenKind::EscU => {
                    let i = get_num!(str_to_u32(&token.content[2..], 16), &token) as u32;
                    let unicode = char::from_u32(i);
                    if let Some(ch) = unicode {
                        let _ = self.buf.write(ch.to_string().as_bytes());
                    }

                    continue;
                }

                TokenKind::EscA => self.buf.push(7),
                TokenKind::EscB => self.buf.push(8),
                TokenKind::EscT => self.buf.push(9),
                TokenKind::EscN => self.buf.push(10),
                TokenKind::EscV => self.buf.push(11),
                TokenKind::EscF => self.buf.push(12),
                TokenKind::EscR => self.buf.push(13),
                TokenKind::EscE => self.buf.push(27),

                _ => {
                    return Err(Error::new(
                        ErrorKind::UnexpectedToken(TokenKind::Text, Some(token.kind)),
                        &token,
                    ));
                }
            };
        };

        let token = self.scanner.scan_token()?;
        expect_token(&token, TokenKind::Identifier)?;
        let mut style = Style::new();
        let tag_name = match token.content {
            // styles
            "b" => {
                style.brightness = Condition::A;
                TagName::B
            }
            "d" => {
                style.brightness = Condition::B;
                TagName::D
            }
            "i" => {
                style.italics = true;
                TagName::I
            }
            "u" => {
                style.under = Condition::A;
                TagName::U
            }
            "k" => {
                style.blink = true;
                TagName::K
            }
            "r" => {
                style.invert = true;
                TagName::R
            }
            "h" => {
                style.hide = true;
                TagName::H
            }
            "s" => {
                style.strike = true;
                TagName::S
            }

            "a" => TagName::A,
            "br" => TagName::Br,
            "c" => TagName::C,
            "e" => TagName::E,
            "let" => TagName::Let,
            "p" => TagName::P,
            "x" => TagName::X,
            "ziyy" => TagName::Ziyy,
            _ => TagName::Any(String::from(token.content)),
        };

        let mut tag = Tag::new(tag_name.clone(), r#type);
        tag.span.add(&token.span);

        let mut token = self.scanner.scan_token()?;
        tag.span.add(&token.span);
        while token.kind == TokenKind::Identifier {
            match token.content {
                // styles
                "b" | "bold" => {
                    assign_prop_cond!(style, brightness, Condition::A, self.scanner, token);
                }
                "d" | "dim" => {
                    assign_prop_cond!(style, brightness, Condition::B, self.scanner, token);
                }
                "i" | "italics" => assign_prop_bool!(style, italics, self.scanner, token),
                "u" | "underline" => {
                    assign_prop_cond!(style, under, Condition::A, self.scanner, token);
                }
                "k" | "blink" => assign_prop_bool!(style, blink, self.scanner, token),
                "r" | "invert" | "reverse" => {
                    assign_prop_bool!(style, invert, self.scanner, token);
                }
                "h" | "hidden" | "hide" | "invisible" => {
                    assign_prop_bool!(style, hide, self.scanner, token);
                }
                "s" | "strike-through" => assign_prop_bool!(style, strike, self.scanner, token),
                "uu" | "double-underline" => {
                    assign_prop_cond!(style, under, Condition::B, self.scanner, token);
                }
                "double" => {
                    if tag_name == TagName::U {
                        assign_prop_cond!(style, under, Condition::B, self.scanner, token);
                    }
                }

                // colors
                "c" | "fg" => {
                    assign_prop_color!(tag, style, fg_color, Foreground, self.scanner, token);
                }
                "x" | "bg" => {
                    assign_prop_color!(tag, style, bg_color, Background, self.scanner, token);
                }
                "black" | "blue" | "cyan" | "green" | "magenta" | "red" | "white" | "yellow" => {
                    if tag_name == TagName::C {
                        style.fg_color = Some(Color::try_from((
                            token.content,
                            Channel::Foreground,
                            token.span.clone(),
                        ))?);
                    } else if tag_name == TagName::X {
                        style.bg_color = Some(Color::try_from((
                            token.content,
                            Channel::Background,
                            token.span.clone(),
                        ))?);
                    }

                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;
                    }
                }
                "byte" => {}
                "rgb" => {}

                // custom
                "n" => {
                    // number of newlines to insert
                    if tag_name == TagName::Br {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "href" => {
                    // url of link
                    if tag_name == TagName::A {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "name" => {
                    // name of binding to declare
                    if tag_name == TagName::Let {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }
                "tab" => {
                    // number of spaces to insert before a paragraph/ a tab if Value::Bool
                    if tag_name == TagName::P {
                        assign_prop_value!(tag, custom, self.scanner, token);
                    }
                }

                // inherit properties from binding with name
                "src" => assign_prop_value!(tag, src, self.scanner, token),

                // ignore unknown properties
                _ => {
                    token = self.scanner.scan_token()?;
                    tag.span.add(&token.span);
                    if token.kind == TokenKind::Equal {
                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                        expect_token(&token, TokenKind::String)?;

                        token = self.scanner.scan_token()?;
                        tag.span.add(&token.span);
                    }
                }
            }
        }

        tag.style = style;

        match token.kind {
            TokenKind::Great => {}
            TokenKind::SlashGreat => {
                tag.r#type = TagType::OpenAndClose;
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedToken(TokenKind::Great, Some(token.kind)),
                    &token,
                ));
            }
        }

        Ok(Chunk::Tag(tag))
    }

    pub(super) fn parse_next_chunk(&mut self) -> Result<Chunk, Error> {
        let chunk = self.parse_chunk()?;
        self.next_chunk = Some(chunk.clone());
        Ok(chunk)
    }
}
