use super::token::Token;
use super::token::TokenType::{self, *};
use crate::common::Span;
use crate::scanner::{GenericScanner, Source};
use crate::splitter::fragment::Fragment;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    span: Span,
}

impl Scanner {
    pub fn new(mut source: Fragment) -> Self {
        source.span.tie_start();

        Self {
            source: source.lexeme.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            span: source.span,
        }
    }

    fn escape(&mut self) {
        if self.is_at_end() {
            return;
        }
        let c = self.advance();

        let mut scan_until = |limit: u8, tester: fn(c: char) -> bool| {
            let mut i = 0;
            while i < limit && tester(self.peek()) {
                self.advance();
                i += 1;
            }
        };

        fn is_hexdigit(c: char) -> bool {
            c.is_ascii_hexdigit()
        }

        fn is_octdigit(c: char) -> bool {
            matches!(c, '0'..'8')
        }

        let r#type = match c {
            'a' => ESCAPE_A,
            'b' => ESCAPE_B,
            'e' => ESCAPE_E,
            'f' => ESCAPE_F,
            'n' => ESCAPE_N,
            'r' => ESCAPE_R,
            't' => ESCAPE_T,
            'v' => ESCAPE_V,
            '\\' => ESCAPE_BACKSLASH,
            '<' => ESCAPE_LESS,
            '>' => ESCAPE_GREATER,
            '0' => {
                scan_until(3, is_octdigit);
                ESCAPE_0
            }
            'x' => {
                scan_until(2, is_hexdigit);
                ESCAPE_X
            }
            'u' => {
                scan_until(4, is_hexdigit);
                ESCAPE_U
            }
            'U' => {
                scan_until(8, is_hexdigit);
                ESCAPE_U
            }
            _ => return self.text(),
        };

        self.add_token(r#type);
    }

    fn text(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                '\\' => break,
                _ => {
                    self.advance();
                }
            }
        }
        self.add_token(TEXT);
    }

    fn add_token(&mut self, r#type: TokenType) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(r#type, text, self.span));
    }
}

impl_generic_scanner!(|s: &mut Scanner| {
    let c = s.advance();
    match c {
        '\\' => s.escape(),
        '\x1B' => {
            s.add_token(ESCAPE);
        }
        _ => s.text(),
    }

    if s.is_at_end() {
        s.add_token(EOF);
    }
});

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for [char] {
    fn to_string(&self) -> String {
        let mut text = String::with_capacity(self.len());

        for ch in self {
            text.push(*ch)
        }

        text
    }
}
