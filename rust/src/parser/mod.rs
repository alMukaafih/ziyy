use state::State;
pub use tag::{Tag, TagKind, TagType};

use crate::color::ansi_style::AnsiStyle;
use crate::error::ErrorKind;
use crate::error::FromError;
use crate::scanner::token::{Token, TokenKind};
use crate::scanner::Scanner;
use crate::Error;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;

mod close_tag;
mod helpers;
mod open_and_close_tag;
mod open_tag;
mod parse_tag;
mod state;
mod tag;
mod write_attribs;

/// Ziyy Parser
pub struct Parser<T: AsRef<[u8]>> {
    pub(super) scanner: Scanner<T>,
    pub(crate) buf: Vec<u8>,
    pub(crate) bindings: Option<HashMap<String, Tag>>,
    state: State,
    first_print_char: bool,

    last_written: TagKind,
    next_tag: Option<Tag>,
}

impl<T: AsRef<[u8]>> Parser<T> {
    /// Creates a new Ziyy Parser.
    pub fn new(source: T, bindings: Option<HashMap<String, Tag>>) -> Parser<T> {
        Parser {
            scanner: Scanner::new(source),
            buf: vec![],
            bindings,
            state: State::new(),
            first_print_char: true,

            last_written: TagKind::None,
            next_tag: None,
        }
    }

    /// Parses Ziyy source and Returns a `&[u8]`.
    pub fn parse_as_slice(&mut self) -> Result<&[u8], Error> {
        let _ = write!(self.buf, "\x1b[m");

        loop {
            let tag = self.parse_tag()?;
            if tag.kind == TagKind::Eof {
                let _ = write!(self.buf, "\x1b[m");
                return Ok(self.buf.as_slice());
            }
            match tag.r#type {
                TagType::Open => self.parse_open_tag(tag)?,

                TagType::Close => self.parse_close_tag(tag)?,

                TagType::OpenAndClose => self.parse_open_and_close_tag(tag)?,
            }
        }
    }

    /// Parses Ziyy source and Returns a [Cow<'_, str>].
    pub fn parse(&mut self) -> Result<Cow<'_, str>, Error> {
        Ok(String::from_utf8_lossy(self.parse_as_slice()?))
    }

    fn expect_tag(tag: &Tag, to_be: TagKind, err: ErrorKind) -> Result<(), Error> {
        if tag.kind != to_be {
            return Err(Error::from_err(err, tag.start.clone(), tag.end.clone()));
        }
        Ok(())
    }

    fn write_and_save(&mut self, tag: TagKind, style: AnsiStyle) {
        let _ = self.buf.write(&style.to_bytes());
        self.state.push(tag.clone(), style);
        if tag == TagKind::Text || tag == TagKind::P {
            self.last_written = tag;
        }
    }
}

fn expect_token(token: &Token, tt: TokenKind) -> Result<(), Error> {
    if token.kind != tt {
        return Err(Error::new(
            ErrorKind::UnexpectedToken {
                expected: tt,
                found: token.kind,
            },
            token.clone(),
        ));
    }
    Ok(())
}

fn inherit(src: &Tag, dst: &mut Tag) {
    if src.b.is_some() && dst.b.is_none() {
        dst.b = Some(None)
    }

    if let Some(ref val) = src.c {
        if dst.c.is_some() {
        } else if let Some(c) = val {
            if !dst.c.as_ref().is_some_and(|x| x.is_some()) {
                dst.c = Some(Some(c.clone()))
            }
        }
    }

    if src.i.is_some() && dst.i.is_none() {
        dst.i = Some(None)
    }

    if src.n.is_some() && dst.n.is_none() {
        dst.n = Some(None)
    }

    if src.s.is_some() && dst.s.is_none() {
        dst.s = Some(None)
    }

    if src.u.is_some() && dst.u.is_none() {
        dst.u = Some(None)
    }

    if let Some(ref val) = src.x {
        if dst.x.is_some() {
        } else if let Some(c) = val {
            if !dst.x.as_ref().is_some_and(|x| x.is_some()) {
                dst.x = Some(Some(c.clone()))
            }
        }
    }

    if let Some(ref tab) = src.tab {
        if dst.tab.is_some() {
        } else if let Some(val) = tab {
            if !dst.tab.as_ref().is_some_and(|x| x.is_some()) {
                dst.tab = Some(Some(val.clone()))
            }
        } else {
            dst.tab = Some(None)
        }
    }
}
