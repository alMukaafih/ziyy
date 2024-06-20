pub mod token;

use crate::scanner::token::*;

pub struct Scanner {
    source: Vec<char>,
    start: i32,
    current: i32,
    line: i32,
    text_mode: bool,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
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

    pub fn make_token(&mut self, kind: TokenKind) -> Token {
        let mut s = String::new();
        for i in self.start..self.current {
            let ch = self.source.get(i as usize);
            if let Some(ch) = ch {
                s.push(*ch);
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

    pub fn scan_token(&mut self) -> Token {
        self.start = self.current;
        if self.is_at_end() { return self.make_token(TokenKind::Eof) }

        let c = self.advance();
        if c == ']' {
            self.text_mode = true;
            return self.make_token(TokenKind::RightSquare)
        }
        
        if self.text_mode {
            while !self.is_at_end() {
                //println!("{c}");
                if self.peek() != '[' {
                    self.advance();
                } else {
                    break
                }
            }
            self.text_mode = false;
            return self.make_token(TokenKind::Text);
        }

        match c {
            '(' => self.make_token(TokenKind::LeftParen),
            ')' => self.make_token(TokenKind::RightParen),
            '{' => self.make_token(TokenKind::LeftBrace),
            '}' => self.make_token(TokenKind::RightBrace),
            '[' => {
                self.make_token(TokenKind::LeftSquare)
            },
            ']' => self.make_token(TokenKind::RightSquare),
            ':' => self.make_token(TokenKind::Colon),
            ';' => self.make_token(TokenKind::SemiColon),
            ',' => self.make_token(TokenKind::Comma),
            '+' => self.make_token(TokenKind::Plus),
            '=' => self.make_token(TokenKind::Equal),
            _   => self.error_token("Unexpected character.")
        }
    }
}