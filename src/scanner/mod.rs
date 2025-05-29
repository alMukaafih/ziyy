pub use source::Source;

use crate::common::Span;

mod source;

pub fn is_alpha(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub fn is_hexdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

pub trait GenericScanner<T: PartialEq, U> {
    fn source(&self) -> &impl Source<T>;
    fn tokens(&mut self) -> &mut Vec<U>;
    fn start(&self) -> usize;
    fn set_start(&mut self, n: usize);
    fn current(&self) -> usize;
    fn set_current(&mut self, n: usize);
    fn span(&mut self) -> &mut Span;
    fn scan_token(&mut self);

    fn scan_tokens<'a>(&'a mut self) -> Vec<U>
    where
        T: 'a,
    {
        while !self.is_at_end() {
            self.set_start(self.current());
            self.scan_token();
        }

        std::mem::take(self.tokens())
    }

    fn peek(&self) -> T {
        if self.is_at_end() {
            self.source().null()
        } else {
            self.source().at(self.current())
        }
    }

    fn peek_next(&self) -> T {
        if self.current() + 1 > self.source().len() {
            self.source().null()
        } else {
            self.source().at(self.current() + 1)
        }
    }

    fn peek_n(&self, n: usize) -> T {
        if self.current() + n > self.source().len() {
            self.source().null()
        } else {
            self.source().at(self.current() + n)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current() >= self.source().len()
    }

    fn advance(&mut self) -> T {
        self.set_current(self.current() + 1);
        *self.span() += (0, 1);
        let ch = self.source().at(self.current() - 1);
        if ch == self.source().nl() {
            *self.span() += (1, 0);
        }
        ch
    }
}

macro_rules! impl_generic_scanner {
    ( $f:expr ) => {
        impl GenericScanner<char, Token> for Scanner {
            fn source(&self) -> &impl Source<char> {
                &self.source
            }

            fn tokens(&mut self) -> &mut Vec<Token> {
                &mut self.tokens
            }

            fn start(&self) -> usize {
                self.start
            }

            fn set_start(&mut self, n: usize) {
                self.start = n;
            }

            fn current(&self) -> usize {
                self.current
            }

            fn set_current(&mut self, n: usize) {
                self.current = n;
            }

            fn span(&mut self) -> &mut Span {
                &mut self.span
            }

            fn scan_token(&mut self) {
                $f(self);
            }
        }
    };
}
