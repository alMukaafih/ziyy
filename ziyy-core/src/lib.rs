#![allow(clippy::pedantic)]
#![warn(missing_docs)]

//! # Ziyy's core library

pub use error::{Error, ErrorType, Result};
pub use indexer::Indexer;
pub use parser::{Parser, WordParser, chunk::Chunk};
pub use resolver::{
    Resolver,
    document::{Document, Node},
};
pub use splitter::{
    Splitter,
    fragment::{Fragment, FragmentType},
};

pub use common::{Position, Span};
pub use parser::color::Color;

mod builtin;
mod error;
#[macro_use]
mod scanner;
mod common;
mod indexer;
mod parser;
mod resolver;
mod splitter;

// mod ziyy;

/// Styles the given text using ziyy.
///
/// # Example
///
/// ```
/// # use ziyy_core as ziyy;
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
/// ```
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input source.
///
pub fn style<T: AsRef<str>>(source: T) -> String {
    match try_style(source) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

/// Styles the given text using ziyy.
pub fn try_style<T: AsRef<str>>(source: T) -> Result<String> {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.as_ref().to_string());
    let mut splitter = Splitter::new();
    #[allow(clippy::unnecessary_to_owned)]
    let frags = splitter.split(source)?;

    let parser = Parser::new(false);
    let chunks = parser.parse(frags)?;

    let mut resolver = Resolver::new(false);
    let output = resolver.resolve(chunks)?;

    let mut buf = String::new();
    output.root().to_string(&mut buf);
    Ok(buf)
}
