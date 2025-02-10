use crate::error::ErrorKind;
use crate::scanner::token::{Token, TokenKind};
use crate::scanner::Scanner;
use crate::style::{Condition, Style};
use crate::Error;
use parse_chunk::Chunk;
use state::State;
use std::collections::HashMap;
use std::io::Write;
pub use tag::{Tag, TagName, TagType};

mod close_tag;
mod helpers;
mod open_and_close_tag;
mod open_tag;
mod parse_chunk;
mod state;
mod tag;

#[derive(PartialEq)]
enum Printable {
    Text,
    Paragraph,
    WhiteSpace,
    None,
}

/// Ziyy Parser
pub struct Parser<T: AsRef<str>> {
    pub(super) scanner: Scanner<T>,
    pub(crate) buf: Vec<u8>,
    pub(crate) bindings: Option<HashMap<String, Style>>,
    state: State,
    /// Should skip white space
    skip_ws: bool,
    /// Should it style the text exactly as it is?
    exact: bool,
    /// Should clear under line?
    clear_under: bool,

    last_written: Printable,
    next_chunk: Option<Chunk>,
}

impl<T: AsRef<str>> Parser<T> {
    /// Creates a new Ziyy Parser.
    pub fn new(source: T, bindings: Option<HashMap<String, Style>>) -> Parser<T> {
        Parser {
            scanner: Scanner::new(source),
            buf: vec![],
            bindings,
            state: State::new(),
            skip_ws: true,
            exact: false,
            clear_under: false,

            last_written: Printable::None,
            next_chunk: None,
        }
    }

    /// Parses Ziyy source and Returns a [Vec<u8>].
    pub fn parse_to_bytes(&mut self) -> Result<Vec<u8>, Error> {
        if !self.exact {
            let _ = write!(self.buf, "\x1b[m");
        }

        loop {
            let parsed = self.parse_chunk()?;
            match parsed {
                Chunk::Tag(tag) => match tag.r#type {
                    TagType::Open => self.parse_open_tag(tag)?,
                    TagType::Close => self.parse_close_tag(tag)?,
                    TagType::OpenAndClose => self.parse_open_and_close_tag(tag)?,
                },

                Chunk::Text(text) => {
                    let _ = self.buf.write(text.as_bytes());
                    self.skip_ws = false;
                    self.last_written = Printable::Text;
                }

                Chunk::WhiteSpace(ws) => {
                    let chunk = self.parse_next_chunk()?;
                    if self.exact {
                        if let Chunk::Text(_) = chunk {
                            let _ = self.buf.write(ws.as_bytes());
                        } else {
                            let _ = match self.state.current_style() {
                                Some(style) if style.under.is_some() => {
                                    let _ = self.buf.write(b"\x1b[24m");
                                    let _ = self.buf.write(ws.as_bytes());
                                    match style.under {
                                        Condition::A | Condition::BA => self.buf.write(b"\x1b[4m"),
                                        Condition::B | Condition::AB => self.buf.write(b"\x1b[21m"),
                                        Condition::None => Ok(0),
                                    }
                                }
                                _ => self.buf.write(ws.as_bytes()),
                            };
                        }
                    } else {
                        if !self.skip_ws && self.last_written != Printable::WhiteSpace {
                            if let Chunk::Text(_) = chunk {
                                let _ = self.buf.write(b" ");
                            } else {
                                let _ = match self.state.current_style() {
                                    Some(style) if style.under.is_some() => {
                                        let _ = self.buf.write(b"\x1b[24m");
                                        let _ = self.buf.write(b" ");
                                        match style.under {
                                            Condition::A | Condition::BA => {
                                                self.buf.write(b"\x1b[4m")
                                            }
                                            Condition::B | Condition::AB => {
                                                self.buf.write(b"\x1b[21m")
                                            }
                                            Condition::None => Ok(0),
                                        }
                                    }
                                    _ => self.buf.write(b" "),
                                };
                            }

                            self.last_written = Printable::WhiteSpace;
                        }

                        match chunk {
                            Chunk::Eof => {
                                if ws.contains('\n') {
                                    let _ = self.buf.write(b"\n");
                                }
                            }
                            _ => {}
                        }
                    }
                }

                Chunk::Eof => {
                    let _ = write!(self.buf, "\x1b[m");
                    return Ok(self.buf.drain(..).collect::<Vec<_>>());
                }
            }
        }
    }

    /// Parses Ziyy source and Returns a [String].
    pub fn parse(&mut self) -> Result<String, Error> {
        let s = String::from_utf8(self.parse_to_bytes()?);
        Ok(s.unwrap())
    }

    fn expect_tag(tag: &Tag, to_be: TagName, err: ErrorKind) -> Result<(), Error> {
        if tag.name != to_be {
            Err(Error {
                kind: err,
                span: tag.span.clone(),
            })
        } else {
            Ok(())
        }
    }

    fn write_and_save(&mut self, tag_name: TagName, style: Style) {
        if let Some(prev) = self.state.current_style() {
            let delta = style.sub(&prev);
            let _ = self.buf.write(&delta.to_string().as_bytes());
            self.state.push(tag_name.clone(), style, delta);
        }
        if tag_name == TagName::P {
            self.last_written = Printable::Paragraph;
        }
    }
}

fn expect_token(token: &Token, tt: TokenKind) -> Result<(), Error> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken(token.kind, Some(tt)),
            &token,
        ));
    }
    Ok(())
}

macro_rules! inherit_style_bool {
    ( $src:expr, $dst:expr, $f:tt ) => {
        if $src.$f && !$dst.$f {
            $dst.$f = true
        }
    };
}

macro_rules! inherit_style_color {
    ( $src:expr, $dst:expr, $f:tt ) => {
        if $src.$f.is_some() && $dst.$f.is_none() {
            $dst.$f = $src.$f.clone()
        }
    };
}

fn inherit(src: &Style, dst: &mut Style) {
    //inherit_style_bool!(src, dst, brightness);
    //inherit_style_bool!(src, dst, dim);
    inherit_style_bool!(src, dst, italics);
    //inherit_style_bool!(src, dst, under);
    inherit_style_bool!(src, dst, blink);
    inherit_style_bool!(src, dst, invert);
    inherit_style_bool!(src, dst, hide);
    inherit_style_bool!(src, dst, strike);
    //inherit_style_bool!(src, dst, double_under);

    inherit_style_color!(src, dst, fg_color);
    inherit_style_color!(src, dst, bg_color);
}
