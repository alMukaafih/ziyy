use std::io::Write;
use parser::Parser;

mod parser;
mod state;

pub struct Compiler<'a, W: Write> {
    parser: Parser<'a, W>,
}

impl<'a, W: Write> Compiler<'a, W> {
    pub fn new(source: &'a str, out: &'a mut W) -> Compiler<'a, W> {
        Compiler {
            parser: Parser::new(source, out),
        }
    }

    pub fn compile(&mut self) {
        self.parser.parse_to_out();
    }
}