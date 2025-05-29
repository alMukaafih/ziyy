#![allow(clippy::pedantic)]
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

pub fn style<T: AsRef<str>>(text: T) -> String {
    let mut indexer = Indexer::new();
    let source = indexer.index(text.as_ref().to_string());
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
