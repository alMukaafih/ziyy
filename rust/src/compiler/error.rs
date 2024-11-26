use std::{error, fmt, num::ParseIntError, str::Utf8Error, string::FromUtf8Error};

use crate::scanner::token::Token;

/// Parse Error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    line: i32,
    column: i32,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self {
            kind: ErrorKind::Utf8Error {
                error_len: value.error_len(),
                valid_up_to: value.valid_up_to(),
            },
            line: 0,
            column: 0,
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(value: FromUtf8Error) -> Self {
        let value: Utf8Error = value.utf8_error();
        Self {
            kind: ErrorKind::Utf8Error {
                error_len: value.error_len(),
                valid_up_to: value.valid_up_to(),
            },
            line: 0,
            column: 0,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        let _ = value;
        Self {
            kind: ErrorKind::InvalidInt,
            line: 0,
            column: 0,
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Self {
            kind: value,
            line: 0,
            column: 0,
        }
    }
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, token: Token) -> Self {
        let error = Self {
            kind,
            line: token.line,
            column: 1,
        };

        error
    }
}

#[non_exhaustive]
#[derive(Debug)]
#[allow(missing_docs)]
pub enum ErrorKind {
    UnexpectedToken,
    UndefinedVariable,
    WrongClosingTag,
    SyntaxError,
    NoVariable,
    UnclosedTag,
    Utf8Error {
        error_len: Option<usize>,
        valid_up_to: usize,
    },
    InvalidInt,
    ExpectedZiyyTag,
    UnexpectedTag,
    UnexpectedEof,
}
