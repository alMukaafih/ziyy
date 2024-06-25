pub mod token;

use crate::scanner::token::*;

pub struct Scanner<'a> {
    source: &'a [u8],
    start: i32,
    current: i32,
    line: i32,
    text_line: i32,
    text_mode: bool,
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Scanner<'a> {
        Scanner {
            source: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
            text_line: 1,
            text_mode: true,
        }
    }

    pub fn is_at_end(&mut self) -> bool {
        self.current as usize +1 > self.source.len()
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current as usize - 1] as char
    }

    pub fn peek(&mut self) -> char {
        if let Some(c) = self.source.get(self.current as usize) {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn peek_next(&mut self) -> char {
        if let Some(c) = self.source.get(self.current as usize + 1) {
            *c as char
        } else {
            '\0'
        }
    }

    pub fn make_token(&mut self, kind: TokenKind) -> Token {
        let mut s = String::new();
        for i in self.start..self.current {
            let ch = self.source.get(i as usize);
            if let Some(ch) = ch {
                s.push(*ch as char);
            }
        }
        Token {
            kind,
            content: s,
            line: self.line,
        }
    }

    pub fn error_token(&self, message: &str) -> Token {
        Token {
            kind: TokenKind::Error,
            content: String::from(message),
            line: self.line,
        }
    }

    pub fn text_token(&mut self) -> Token {
        let mut s = String::new();
        for i in self.start..self.current {
            let ch = self.source.get(i as usize);
            if let Some(ch) = ch {
                s.push(*ch as char);
            }
        }
        Token {
            kind: TokenKind::Text,
            content: s,
            line: self.text_line,
        }
    }

    pub fn skip_whitespace(&mut self) {
        loop {
            if self.text_mode {return;}
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.text_line += 1;
                    self.advance();
                    break;
                }
                _ => return
            }

        }
    }

    pub fn identifier_kind(&mut self) -> TokenKind {
        use TokenKind::*;
        if self.current - self.start == 1 {
            match self.source[self.start as usize] as char {
                'b' => return B,
                'c' => return C,
                'i' => return I,
                's' => return S,
                'u' => return U,
                'x' => return X,
                _ => return Identifier,
            }
        } else {
            Identifier
        }
    }

    pub fn identifier(&mut self) -> Token {
        while is_alpha(self.peek()) || is_digit(self.peek()) {
            self.advance();
        }

        let kind = self.identifier_kind();
        self.make_token(kind)
    }

    pub fn number(&mut self) -> Token {
        while is_digit(self.peek()) { self.advance(); }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();

            while is_digit(self.peek()) { self.advance(); };

        }
        self.make_token(TokenKind::Number)
    }

    // pub fn string(&mut self) -> Token {
    //     while self.peek() != ']' && self.peek() != '"' && !self.is_at_end() {
    //         if self.peek() != '\n' {
    //             self.line += 1;
    //             self.text_line += 1;
    //         }
    //         self.advance();
    //     }

    //     if self.peek() == '%' && self.peek_next() == '>' || self.is_at_end() {
    //         return self.error_token("Unterminated string.");
    //     }

    //     self.advance();
    //     self.make_token(TokenKind::String)
    // }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() { return self.make_token(TokenKind::Eof) }

        let c = self.advance();
        if c == '<' {
            self.text_mode = false;
            return self.make_token(TokenKind::OpenTag)
        }
        else if c =='>' {
            self.text_mode = true;
            return self.make_token(TokenKind::CloseTag)
        }

        if self.text_mode {
            while !self.is_at_end() {
                if self.peek() == '\n' {
                    self.line += 1;
                    self.text_line += 1;
                }
                if self.peek() == '\\' {
                    self.advance();
                    self.advance();
                    continue
                }
                if self.peek() != '<' {
                    self.advance();
                }
                else {
                    break
                }
            }
            return self.text_token();
        }

        if is_alpha(c) { return  self.identifier() }
        if is_digit(c) { return self.number() }

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
            // '=' => self.make_token(TokenKind::Equal),
            //'"' => self.string(),
            '/' => self.make_token(TokenKind::Slash),
            _   => self.error_token("Unexpected character.")
        }
    }
}