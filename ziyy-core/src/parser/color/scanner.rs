use crate::common::Span;
use crate::scanner::{GenericScanner, Source, is_alpha, is_alpha_numeric, is_digit, is_hexdigit};

use super::token::Token;
use super::token::TokenType::{self, *};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

pub static COLORS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    [
        ("bblack", BG_BLACK),
        ("bblue", BG_BLUE),
        ("bbyte", BG_FIXED),
        ("bcyan", BG_CYAN),
        ("bdefault", BG_DEFAULT),
        ("bfalse", BG_DEFAULT),
        ("bnone", BG_DEFAULT),
        ("bgreen", BG_GREEN),
        ("bmagenta", BG_MAGENTA),
        ("bred", BG_RED),
        ("brgb", BG_RGB),
        ("bwhite", BG_WHITE),
        ("byellow", BG_YELLOW),
        ("fblack", FG_BLACK),
        ("fblue", FG_BLUE),
        ("fbyte", FG_FIXED),
        ("fcyan", FG_CYAN),
        ("fdefault", FG_DEFAULT),
        ("ffalse", FG_DEFAULT),
        ("fnone", FG_DEFAULT),
        ("fgreen", FG_GREEN),
        ("fmagenta", FG_MAGENTA),
        ("fred", FG_RED),
        ("frgb", FG_RGB),
        ("fwhite", FG_WHITE),
        ("fyellow", FG_YELLOW),
    ]
    .into()
});

macro_rules! shrink {
    ($num:expr) => {{
        if $num > 255.0 {
            255
        } else if $num < 0.0 {
            0
        } else {
            $num as u8
        }
    }};
}

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    span: Span,
}

impl_generic_scanner!(|s: &mut Scanner| {
    let c = s.advance();
    match c {
        '(' => s.add_token(LEFT_PAREN),
        ')' => s.add_token(RIGHT_PAREN),
        ',' => s.add_token(COMMA),
        '{' => s.place_holder(),
        c => {
            if c == 'b' && s.peek() == '#' {
                s.advance();
                s.hex(BG_HEX);
            } else if c == 'f' && s.peek() == '#' {
                s.advance();
                s.hex(FG_HEX);
            } else if is_digit(c) {
                s.number();
            } else if is_alpha(c) {
                s.identifier();
            } else {
                // TODO: error
            }
        }
    }
});

impl Scanner {
    pub fn new(source: String, mut span: Span) -> Self {
        span.tie_start();

        Self {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            span,
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let k = self.source[self.start..self.current].to_string();

        if let Some(r#type) = COLORS.get(k.as_str()) {
            self.add_token(*r#type);
        } else {
            self.add_token(IDENTIFIER);
        }
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].to_string();
        self.add_token2(
            NUMBER,
            Some(f64::from_str(value.as_str()).unwrap().round()).map(|x| shrink!(x)),
        );
    }

    fn place_holder(&mut self) {
        loop {
            if self.peek() == '}' && self.peek_next() == '}' {
                self.advance();
            } else if self.peek() == '}' || self.is_at_end() {
                break;
            }
            self.advance();
        }

        if self.is_at_end() {
            // TODO: error
            return;
        }

        self.advance();

        self.add_token(PLACE_HOLDER);
    }

    fn hex(&mut self, r#type: TokenType) {
        while is_hexdigit(self.peek()) {
            self.advance();
        }
        self.add_token(r#type);
    }

    fn add_token(&mut self, r#type: TokenType) {
        self.add_token2(r#type, None);
    }

    fn add_token2(&mut self, r#type: TokenType, literal: Option<u8>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(r#type, text, literal, self.span));
    }
}

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
