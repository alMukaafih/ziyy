use std::mem::take;

use fragment::Fragment;
use fragment::FragmentType::{self, *};

use crate::common::Span;

pub mod fragment;

#[doc(hidden)]
pub struct Splitter {
    source: Vec<char>,
    fragments: Vec<Fragment>,
    start: usize,
    current: usize,
    span: Span,
}

impl Default for Splitter {
    fn default() -> Self {
        Self::new()
    }
}

enum Quote {
    Single,
    Double,
    None,
}

impl Splitter {
    pub fn new() -> Self {
        Self {
            source: vec![],
            fragments: vec![],
            start: 0,
            current: 0,
            span: Span::default(),
        }
    }

    pub fn split(&mut self, source: String) -> Vec<Fragment> {
        self.source = source.chars().collect();

        macro_rules! consume_word {
            ($c:ident) => {
                loop {
                    if self.is_at_end() {
                        break;
                    }

                    if is_whitespace(self.peek()) {
                        break;
                    }

                    if matches!(self.peek(), '<') {
                        break;
                    }

                    if matches!($c, '\\') {
                        self.advance();
                    }

                    self.advance();
                }
            };
        }

        while !self.is_at_end() {
            self.start = self.current;

            let mut c = self.advance();

            match c {
                ' ' | '\r' | '\t' | '\n' => self.whitespace(),
                '\\' => {
                    c = self.advance();
                    consume_word!(c);
                    self.add_fragment(Word);
                }
                '<' => self.tag(),
                _ => {
                    consume_word!(c);
                    self.add_fragment(Word);
                }
            }
        }

        take(&mut self.fragments)
    }

    fn tag(&mut self) {
        let mut quote = Quote::None;

        loop {
            let c = self.advance();
            if self.is_at_end() {
                self.add_fragment(Error);
                return;
            }

            let close = matches!(self.peek(), '>');
            let single = matches!(self.peek(), '\'');
            let double = matches!(self.peek(), '"');
            let esc = matches!(c, '\\');
            match quote {
                Quote::Single => {
                    if single && !esc {
                        quote = Quote::None;
                    }
                }
                Quote::Double => {
                    if double && !esc {
                        quote = Quote::None;
                    }
                }
                Quote::None => {
                    if close {
                        break;
                    } else if single {
                        quote = Quote::Single;
                    } else if double {
                        quote = Quote::Double;
                    }
                }
            }
        }

        self.advance();
        self.add_fragment(Tag);
    }

    fn whitespace(&mut self) {
        while is_whitespace(self.peek()) {
            self.advance();
        }
        self.add_fragment(Whitespace);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.span += (0, 1);
        let ch = self.source[self.current - 1];
        if ch == '\n' {
            self.span += (1, 0);
        }
        ch
    }

    fn add_fragment(&mut self, r#type: FragmentType) {
        let text = self.source[self.start..self.current].to_string();
        self.fragments.push(Fragment::new(r#type, text, self.span));
        self.span.tie_end();
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

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}
