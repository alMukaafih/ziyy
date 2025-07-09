use std::rc::Rc;

pub use parse::ParserOptions;

use crate::{Document, Indexer, Parser, Resolver, Splitter};

mod parse;
mod render;

pub struct Ziyy {
    doc: Rc<Document>,
}

impl Ziyy {
    pub fn parse(source: String, options: ParserOptions) -> crate::Result<Self> {
        let mut indexer = Indexer::new();
        let source = indexer.index(source);
        let mut splitter = Splitter::new();
        let frags = splitter.split(source);

        let parser = Parser::new(false);
        let chunks = match parser.parse(frags) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };

        let mut resolver = Resolver::new(false);
        Ok(Self {
            doc: resolver.resolve(chunks)?,
        })
    }

    pub fn render(&mut self) {}
}
