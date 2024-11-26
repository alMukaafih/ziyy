use std::io::{stdout, Write};

use ziyy::scanner::{Scanner, token::*};


fn debug(source: &str, out: &mut impl Write) {
    let mut scanner = Scanner::new(source);
    let mut line = -1;
    loop {
        let token = scanner.scan_token().unwrap();
        let content = if token.err_code == 0 {
            token.content
        } else {
            "Unexpected character."
        };
        if token.line != line {
            let _ = write!(out, "{:4} ", token.line);
            line = token.line;
        } else {
            let _ = write!(out, "   | ");
        }
        let _ = writeln!(out, "{:?} '{}'", token.kind, content);
        if token.kind == TokenKind::Eof { break };
    }
}

fn main() {
    let mut out = stdout();
    debug(include_str!("../../.././help.zi"), &mut out)
}