use std::mem::take;

use fragment::Fragment;
use fragment::FragmentType::{self, *};

pub mod fragment;

pub struct Stage2 {
    source: Vec<char>,
    fragments: Vec<Fragment>,
    start: usize,
    current: usize,
    line: usize,
}

impl Default for Stage2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Stage2 {
    pub fn new() -> Self {
        Self {
            source: vec![],
            fragments: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn parse(&mut self, source: String) -> Vec<Fragment> {
        self.source = source.chars().collect();

        while !self.is_at_end() {
            self.start = self.current;

            let c = self.advance();

            match c {
                ' ' | '\r' | '\t' | '\n' => self.whitespace(),
                '\\' => {
                    self.advance();
                    self.add_fragment(Word);
                }
                '<' => self.tag(),
                _ => {
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

                        self.advance();
                    }
                    self.add_fragment(Word);
                }
            }
        }

        let frags = take(&mut self.fragments);
        let mut new_frags = vec![];
        let mut skip = 0;
        for (i, frag) in frags.iter().enumerate() {
            if skip > 0 {
                skip -= 1;
                continue;
            }

            if matches!(frag.r#type, Word) {
                let mut frag = frag.clone();
                let mut n = i + 1;

                while n < frags.len() {
                    let f = frags[n].clone();
                    if matches!(f.r#type, Word) {
                        frag.lexeme += f.lexeme.as_str();
                        n += 1;
                    } else {
                        new_frags.push(frag);
                        skip = n - i - 1;
                        break;
                    }
                }
            } else {
                new_frags.push(frag.clone());
            }
        }

        new_frags
    }

    fn tag(&mut self) {
        enum Quote {
            Single,
            Double,
            None,
        }
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
        let ch = self.source[self.current - 1];
        if ch == '\n' {
            self.line += 1;
        }
        ch
    }

    fn add_fragment(&mut self, r#type: FragmentType) {
        let text = self.source[self.start..self.current].to_string();
        self.fragments.push(Fragment::new(r#type, text, self.line));
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
