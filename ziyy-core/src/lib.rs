#![allow(clippy::pedantic)]
pub use error::{Error, ErrorType, Result};
pub use indexer::Indexer;
pub use parser::Parser;
pub use resolver::Resolver;
pub use splitter::Splitter;

mod error;
#[macro_use]
mod scanner;
mod common;
mod indexer;
mod parser;
mod resolver;
mod splitter;

/// Styles the given text using ziyy.
///
/// # Example
///
/// ```
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
/// ```
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input source.
///
pub fn style<T: AsRef<str>>(source: T) -> String {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.as_ref().to_string());
    let mut splitter = Splitter::new();
    let frags = splitter.split(source);

    let parser = Parser::new();
    let chunks = parser.parse(frags).unwrap(); // TODO: better panics on message

    let mut resolver = Resolver::new();
    let output = resolver.resolve(chunks);

    let mut buf = String::new();
    output.root().to_string(&mut buf);
    buf
}
