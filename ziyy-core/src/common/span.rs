use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub},
};

use super::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Span {
    start: Position,
    end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn tie_end(&mut self) {
        self.start = self.end;
    }

    pub fn tie_start(&mut self) {
        self.end = self.start;
    }

    pub fn unquote(&self) -> Self {
        let mut span = *self;
        span.start += (0, 1);
        span.end -= (0, 1);
        span
    }

    pub fn inserted() -> Self {
        let pos = Position::new(0, 0);
        Span::new(pos, pos)
    }

    pub fn calculate(s: &str) -> Self {
        let start = Position::default();

        let mut end = start;
        for c in s.chars() {
            if c == '\n' {
                end.line += 1;
                end.column = 1;
            } else {
                end.column += 1;
            }
        }

        Self { start, end }
    }
}

impl Add<(usize, usize)> for Span {
    type Output = Span;

    fn add(mut self, rhs: (usize, usize)) -> Self::Output {
        self.end += rhs;
        self
    }
}

impl AddAssign<(usize, usize)> for Span {
    fn add_assign(&mut self, rhs: (usize, usize)) {
        self.end += rhs;
    }
}

impl AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.end = rhs.end
    }
}

impl Sub<(usize, usize)> for Span {
    type Output = Span;

    fn sub(mut self, rhs: (usize, usize)) -> Self::Output {
        self.start -= rhs;
        self
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("{:#}..{:#}", self.start, self.end))
        } else if *self == Span::inserted() {
            f.write_str("\x1b[4minserted\x1b[24m")
        } else {
            f.write_fmt(format_args!("{}..{}", self.start, self.end))
        }
    }
}
