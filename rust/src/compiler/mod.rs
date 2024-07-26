use parser::Parser;
use std::{collections::HashMap, io::Write};

mod parser;
mod state;

/// Compiler.
pub struct Compiler<'a, W: Write> {
    parser: Parser<'a, W>,
}

impl<'a, W: Write> Compiler<'a, W> {
    /// Creates a new Compiler.
    pub fn new(
        source: &'a str,
        out: &'a mut W,
        variables: HashMap<String, String>,
    ) -> Compiler<'a, W> {
        Compiler {
            parser: Parser::new(source, out, variables),
        }
    }

    /// Compile source.
    pub fn compile(&mut self) {
        let result = self.parser.parse_to_out();
        match result {
            Ok(_) => {},
            Err(err) => {
                panic!("{}", err.get_message())
            },
        }
    }

    /// Compile source.
    pub fn compile_source(&mut self, source: &'a str) {
        self.parser.scanner.source = source.as_bytes();
        self.compile();
    }
}
