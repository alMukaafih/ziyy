pub mod position;
pub mod token;

use position::Position;

use crate::{
    scanner::token::{Token, TokenKind},
    Result,
};
use core::str;
use std::ops::Index;

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_valid(c: char) -> bool {
    c == ':' || c == '-' || c == '.'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn is_octdigit(c: char) -> bool {
    match c {
        '0'..'8' => true,
        _ => false,
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

pub struct Scanner<T: AsRef<str>> {
    pub(crate) source: T,
    start: u16,
    current: u16,

    start_line: u16,
    current_line: u16,

    start_column: u16,
    current_column: u16,

    pub(crate) text_mode: bool,

    pub(crate) parse_colors: bool,
    pub(crate) parse_styles: bool,
}

impl<T: AsRef<str>> Scanner<T> {
    pub fn new(source: T) -> Scanner<T> {
        Scanner {
            source,
            start: 0,
            current: 0,

            start_line: 0,
            current_line: 0,

            start_column: 0,
            current_column: 0,

            text_mode: true,
            parse_colors: false,
            parse_styles: false,
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current as usize + 1 > self.source.as_ref().len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.current_column += 1;
        let ch = self.source.as_ref().as_bytes()[self.current as usize - 1] as char;

        if ch == '\n' {
            self.current_line += 1;
            self.current_column = 0;
        }

        ch
    }

    pub fn peek(&mut self) -> char {
        if let Some(c) = self.source.as_ref().as_bytes().get(self.current as usize) {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn peek_next(&mut self) -> char {
        if let Some(c) = self
            .source
            .as_ref()
            .as_bytes()
            .get(self.current as usize + 1)
        {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn make_token(&mut self, kind: TokenKind) -> Result<Token<'_>> {
        let s = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let start_pos = Position::new(self.start_line, self.start_column);
        let end_pos = Position::new(self.current_line, self.current_column);

        Ok(Token {
            kind,
            content: s,
            err_code: 0,
            start_pos,
            end_pos,
        })
    }

    pub fn error_token(&self, code: u8) -> Result<Token<'_>> {
        let s = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let start_pos = Position::new(self.start_line, self.start_column);
        let end_pos = Position::new(self.current_line, self.current_column);

        Ok(Token {
            kind: TokenKind::Error,
            content: s,
            err_code: code,
            start_pos,
            end_pos,
        })
    }

    pub fn text_token(&mut self) -> Result<Token<'_>> {
        self.make_token(TokenKind::Text)
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.text_mode {
                return;
            }
            let c = self.peek();
            if is_whitespace(c) {
                self.advance();
                continue;
            }

            return;
        }
    }

    pub fn check_keyword(
        &mut self,
        start: u16,
        length: u16,
        rest: &str,
        kind: TokenKind,
    ) -> TokenKind {
        let s = &self.source.as_ref()[((self.start + start) as usize)..(self.current as usize)];
        if self.current - self.start == start + length && s == rest {
            kind
        } else {
            TokenKind::Identifier
        }
    }

    pub fn identifier_kind(&mut self) -> TokenKind {
        use TokenKind::{Identifier, B, C, I, S, T, U, X};
        if self.parse_styles && self.current - self.start == 1 {
            match self.source.as_ref().as_bytes()[self.start as usize] as char {
                'b' => B,
                'c' => C,
                'i' => I,
                's' => S,
                't' => T,
                'u' => U,
                'x' => X,
                _ => Identifier,
            }
        } else if self.parse_colors {
            match self.source.as_ref().as_bytes()[self.start as usize] as char {
                'b' => match self.source.as_ref().as_bytes()[self.start as usize + 1] as char {
                    'l' => match self.source.as_ref().as_bytes()[self.start as usize + 2] as char {
                        'a' => self.check_keyword(3, 2, "ck", TokenKind::Black),
                        'u' => self.check_keyword(3, 1, "e", TokenKind::Blue),
                        _ => Identifier,
                    },
                    'y' => self.check_keyword(2, 2, "te", TokenKind::Byte),
                    _ => Identifier,
                },
                'c' => self.check_keyword(1, 3, "yan", TokenKind::Cyan),
                'g' => self.check_keyword(1, 4, "reen", TokenKind::Green),
                'm' => self.check_keyword(1, 6, "agenta", TokenKind::Magenta),
                'r' => match self.source.as_ref().as_bytes()[self.start as usize + 1] as char {
                    'e' => self.check_keyword(2, 1, "d", TokenKind::Red),
                    'g' => self.check_keyword(2, 1, "b", TokenKind::Rgb),
                    _ => Identifier,
                },
                'w' => self.check_keyword(1, 4, "hite", TokenKind::White),
                'y' => self.check_keyword(1, 5, "ellow", TokenKind::Yellow),
                _ => Identifier,
            }
        } else {
            Identifier
        }
    }

    pub fn identifier(&mut self) -> Result<Token<'_>> {
        while is_alpha(self.peek()) || is_digit(self.peek()) || is_valid(self.peek()) {
            self.advance();
        }

        let kind = self.identifier_kind();
        self.make_token(kind)
    }

    pub fn hex(&mut self) -> Result<Token<'_>> {
        while is_hexdigit(self.peek()) {
            self.advance();
        }

        self.make_token(TokenKind::Hex)
    }

    pub fn number(&mut self) -> Result<Token<'_>> {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }
        self.make_token(TokenKind::Number)
    }

    pub fn string(&mut self, ch: char) -> Result<Token<'_>> {
        while self.peek() != ch && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(2);
        }

        self.advance();
        self.make_token(TokenKind::String)
    }

    pub fn whitespace(&mut self) -> Result<Token<'_>> {
        while is_whitespace(self.peek()) {
            self.advance();
        }

        self.make_token(TokenKind::WhiteSpace)
    }

    pub fn escape(&mut self) -> Result<Token<'_>> {
        let c = self.advance();
        let mut scan_until = |limit: u8| {
            let mut i = 0;
            while i < limit && is_hexdigit(self.peek()) {
                self.advance();
                i += 1;
            }
        };

        let kind = match c {
            'a' => TokenKind::EscA,
            'b' => TokenKind::EscB,
            'e' => TokenKind::EscE,
            'f' => TokenKind::EscF,
            'n' => TokenKind::EscN,
            'r' => TokenKind::EscR,
            't' => TokenKind::EscT,
            'v' => TokenKind::EscV,

            '0' => {
                let mut i = 0;
                while i < 3 && is_octdigit(self.peek()) {
                    self.advance();
                    i += 1;
                }
                TokenKind::Esc0
            }

            'x' => {
                scan_until(2);
                TokenKind::EscX
            }
            'u' => {
                scan_until(4);
                TokenKind::EscU
            }
            _ => {
                return self.error_token(0x1b);
            }
        };

        self.make_token(kind)
    }

    pub fn scan_token(&mut self) -> Result<Token<'_>> {
        self.skip_whitespace();

        self.start = self.current;
        self.start_line = self.current_line;
        self.start_column = self.current_column;

        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance();
        if c == '<' {
            self.text_mode = false;
            if self.peek() == '/' {
                self.advance();
                return self.make_token(TokenKind::OpenTagAndSlash);
            }

            return self.make_token(TokenKind::OpenTag);
        }

        if c == '>' {
            self.text_mode = true;
            return self.make_token(TokenKind::CloseTag);
        }

        if self.text_mode {
            if is_whitespace(c) {
                return self.whitespace();
            }

            if c == '\\' {
                return self.escape();
            }

            while !self.is_at_end() {
                if self.peek() == '\\' {
                    break;
                }

                if is_whitespace(self.peek()) {
                    break;
                }

                if self.peek() != '<' {
                    self.advance();
                } else {
                    break;
                }
            }
            return self.text_token();
        }

        if is_alpha(c) {
            return self.identifier();
        }
        if is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            // '[' => self.make_token(TokenKind::LeftSquare),
            // ']' => self.make_token(TokenKind::RightBrace),
            // '{' => self.make_token(TokenKind::LeftBrace),
            // '}' => self.make_token(TokenKind::RightBrace),
            // ':' => self.make_token(TokenKind::Colon),
            // ';' => self.make_token(TokenKind::SemiColon),
            ',' => self.make_token(TokenKind::Comma),
            '.' => self.make_token(TokenKind::Dot),
            // '+' => self.make_token(TokenKind::Plus),
            '=' => self.make_token(TokenKind::Equal),
            '"' => self.string('"'),
            '\'' => self.string('\''),
            '/' => match self.peek() {
                '>' => {
                    self.advance();
                    self.text_mode = true;
                    self.make_token(TokenKind::SlashAndCloseTag)
                }
                _ => self.make_token(TokenKind::Slash),
            },
            _ => {
                if self.parse_colors && c == '#' {
                    self.hex()
                } else {
                    self.error_token(1)
                }
            }
        }
    }

    pub(crate) fn set_source(&mut self, source: T) {
        self.source = source;
        self.start = 0;
        self.current = 0;

        self.start_line = 0;
        self.current_line = 0;

        self.start_column = 0;
        self.current_column = 0;

        self.text_mode = true;
    }
}
