pub mod token;

use crate::{scanner::token::*, Result};
use std::str;

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_valid(c: char) -> bool {
    c == ':' || c == '-'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_hex(c: char) -> bool {
    c.is_ascii_digit()
        || c == 'a'
        || c == 'b'
        || c == 'c'
        || c == 'd'
        || c == 'e'
        || c == 'f'
        || c == 'A'
        || c == 'B'
        || c == 'C'
        || c == 'D'
        || c == 'E'
        || c == 'F'
}

pub struct Scanner<B: AsRef<[u8]>> {
    pub(crate) source: B,
    start: i32,
    current: i32,
    line: i32,
    text_line: i32,
    pub(crate) text_mode: bool,
    escape: u8,
    pub(crate) parse_colors: bool,
    pub(crate) parse_styles: bool,
}

impl<B: AsRef<[u8]>> Scanner<B> {
    pub fn new(source: B) -> Scanner<B> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            text_line: 1,
            text_mode: true,
            escape: 0,
            parse_colors: false,
            parse_styles: false,
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current as usize + 1 > self.source.as_ref().len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_ref()[self.current as usize - 1] as char
    }

    pub fn peek(&mut self) -> char {
        if let Some(c) = self.source.as_ref().get(self.current as usize) {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn peek_next(&mut self) -> char {
        if let Some(c) = self.source.as_ref().get(self.current as usize + 1) {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn make_token(&mut self, kind: TokenKind) -> Result<Token<'_>> {
        let sl = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let s = str::from_utf8(sl)?;
        Ok(Token {
            kind,
            content: s,
            err_code: 0,
            line: self.line,
        })
    }

    pub fn error_token(&self, code: u8) -> Result<Token<'_>> {
        let sl = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let s = str::from_utf8(sl)?;
        Ok(Token {
            kind: TokenKind::Error,
            content: s,
            err_code: code,
            line: self.line,
        })
    }

    pub fn text_token(&mut self) -> Result<Token<'_>> {
        let sl = &self.source.as_ref()[(self.start as usize)..(self.current as usize)];
        let s = str::from_utf8(sl)?;
        let token = Token {
            kind: TokenKind::Text,
            content: s,
            err_code: 0,
            line: self.text_line,
        };
        self.text_line = self.line;
        Ok(token)
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.text_mode {
                return;
            }
            let c = self.peek();
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.text_line += 1;
                    self.advance();
                    continue;
                } else {
                    self.advance();
                    continue;
                }
            } else {
                return;
            }
        }
    }

    pub fn check_keyword(
        &mut self,
        start: i32,
        length: i32,
        rest: &str,
        kind: TokenKind,
    ) -> TokenKind {
        let sl = &self.source.as_ref()[((self.start + start) as usize)..(self.current as usize)];
        let s = str::from_utf8(sl).unwrap();
        if self.current - self.start == start + length && s == rest {
            kind
        } else {
            TokenKind::Identifier
        }
    }

    pub fn identifier_kind(&mut self) -> TokenKind {
        use TokenKind::*;
        if self.parse_styles && self.current - self.start == 1 {
            match self.source.as_ref()[self.start as usize] as char {
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
            match self.source.as_ref()[self.start as usize] as char {
                'b' => match self.source.as_ref()[self.start as usize + 1] as char {
                    'l' => match self.source.as_ref()[self.start as usize + 2] as char {
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
                'r' => match self.source.as_ref()[self.start as usize + 1] as char {
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


    pub fn hex<>(&mut self) -> Result<Token<'_>> {
        while is_hex(self.peek()) {
            self.advance();
        }

        self.make_token(TokenKind::Hex)
    }


    pub fn number(&mut self) -> Result<Token<'_>> {
        while is_digit(self.peek()) {
            self.advance();
        }

        /* if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) { self.advance(); };

        } */
        self.make_token(TokenKind::Number)
    }

    pub fn string(&mut self, ch: char) -> Result<Token<'_>> {
        while self.peek() != ch && !self.is_at_end() {
            if self.peek() != '\n' {
                self.line += 1;
                self.text_line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.error_token(2);
        }

        self.advance();
        self.make_token(TokenKind::String)
    }

    pub fn scan_token(&mut self) -> Result<Token<'_>> {
        if self.escape == 0 {
            self.skip_whitespace();
        }
        self.start = self.current;
        if self.escape == 2 && self.peek() == '\\' {
            self.escape = 1;
            self.advance();
            return self.make_token(TokenKind::BackSlash);
        }
        if self.escape == 1 {
            self.escape = 0;
            self.advance();
            return self.text_token();
        }
        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }

        let c = self.advance();
        if c == '<' {
            self.text_mode = false;
            if self.peek() == '/' {
                self.advance();
                return self.make_token(TokenKind::OpenTagAndSlash);
            } else {
                return self.make_token(TokenKind::OpenTag);
            }
        } else if c == '>' {
            self.text_mode = true;
            return self.make_token(TokenKind::CloseTag);
        }

        if self.text_mode {
            while !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1
                }
                if self.peek() == '\\' {
                    self.escape = 2;
                    return self.text_token();
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
            },
        }
    }

    pub(crate) fn set_source(&mut self, source: B) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
        self.text_line = 1;
        self.text_mode = true;
        self.escape = 0;
    }
}
