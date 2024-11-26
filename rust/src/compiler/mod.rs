pub use error::{Error, ErrorKind};
use std::{collections::HashMap, io::Write};

use crate::{parser::{Parser, Tag}, Result};

pub(crate) mod error;

/// Compiler.
pub struct Compiler<B: AsRef<[u8]>, W: Write> {
    pub(crate) parser: Parser<B, W>,
    pub(crate) template: Option<W>,
}

impl<B: AsRef<[u8]>, W: Write> Compiler<B, W> {
    /// Creates a new Compiler.
    pub fn new(source: B, out: W, bindings: Option<HashMap<String, Tag>>) -> Compiler<B, W> {
        Compiler {
            parser: Parser::new(source, out, bindings),
            template: None,
        }
    }

    /// Compile source.
    pub fn compile(&mut self) -> Result<()> {
        self.parser.parse()
    }

    /// Compile source.
    pub fn compile_source(&mut self, source: B) -> Result<()> {
        self.parser.scanner.set_source(source);
        self.compile()
    }
}

impl Compiler<String, Vec<u8>> {
    pub(crate) fn compile_template(&mut self) -> Result<()> {
        self.compile()?;
        self.template = Some(self.parser.out.clone());
        Ok(())
    }

    pub(crate) fn compile_rest<R: AsRef<str>>(&mut self, rest: R) -> Result<()> {
        self.parser.out = self.template.clone().unwrap();
        self.compile_source(rest.as_ref().to_owned())
    }
}
