use std::{
    error, fmt,
    num::{ParseFloatError, ParseIntError},
    str::Utf8Error,
    string::FromUtf8Error,
};

use crate::{
    scanner::{
        position::Position,
        token::{Token, TokenKind},
    },
    TagKind,
};

/// Parse Error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    start: Position,
    end: Position,
}

pub trait FromError<E> {
    fn from_err(value: E, start: Position, end: Position) -> Self;
    fn convert<T>(value: Result<T, E>, start: Position, end: Position) -> Result<T, Self>
    where
        Self: Sized,
    {
        match value {
            Ok(x) => Ok(x),
            Err(e) => Err(Self::from_err(e, start, end)),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl FromError<Utf8Error> for Error {
    fn from_err(value: Utf8Error, start: Position, end: Position) -> Self {
        Self {
            kind: ErrorKind::Utf8Error {
                error_len: value.error_len(),
                valid_up_to: value.valid_up_to(),
            },
            start,
            end,
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        let value = value.utf8_error();
        Self {
            kind: ErrorKind::Utf8Error {
                error_len: value.error_len(),
                valid_up_to: value.valid_up_to(),
            },
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }
    }
}

impl FromError<ParseIntError> for Error {
    fn from_err(value: ParseIntError, start: Position, end: Position) -> Self {
        let _ = value;
        Self {
            kind: ErrorKind::InvalidNumber,
            start,
            end,
        }
    }
}

impl FromError<ParseFloatError> for Error {
    fn from_err(value: ParseFloatError, start: Position, end: Position) -> Self {
        let _ = value;
        Self {
            kind: ErrorKind::InvalidNumber,
            start,
            end,
        }
    }
}

impl FromError<ErrorKind> for Error {
    fn from_err(value: ErrorKind, start: Position, end: Position) -> Self {
        Self {
            kind: value,
            start,
            end,
        }
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, token: Token) -> Self {
        Self {
            kind,
            start: token.start_pos,
            end: token.end_pos,
        }
    }
}

#[non_exhaustive]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum ErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    WrongClosingTag {
        expected: TagKind,
        found: TagKind,
    },
    Utf8Error {
        error_len: Option<usize>,
        valid_up_to: usize,
    },
    InvalidNumber,
    UnexpectedTag,
    UnexpectedEof,
}
