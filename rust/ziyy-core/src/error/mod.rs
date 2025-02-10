use std::{error, fmt};

use crate::{
    scanner::{
        span::Span,
        token::{Token, TokenKind},
    },
    TagName,
};

/// Parse Error.
#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) span: Span,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnexpectedToken(found, expected) => {
                if let Some(expected) = expected {
                    f.write_fmt(format_args!(
                        "Unexpected Token: {found:?}, expected {expected:?} at {}",
                        self.span
                    ))?;
                } else {
                    f.write_fmt(format_args!("Unexpected Token: {found:?} at {}", self.span))?;
                }
            }
            ErrorKind::MisMatchedTags(open, close) => f.write_fmt(format_args!(
                "Mismatched closing tag </{close:?}> for <{open:?}> at {}",
                self.span
            ))?,
            ErrorKind::InvalidNumber => {
                f.write_fmt(format_args!("Invalid Number at {}", self.span))?;
            }
            ErrorKind::UnexpectedEof => {
                f.write_fmt(format_args!("Unexpected Eof at {}", self.span))?;
            }
        }
        Ok(())
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, token: &Token) -> Self {
        Self {
            kind,
            span: token.span.clone(),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, PartialEq)]
#[allow(missing_docs)]
pub enum ErrorKind {
    UnexpectedToken(TokenKind, Option<TokenKind>),
    MisMatchedTags(TagName, TagName),
    InvalidNumber,
    UnexpectedEof,
}
