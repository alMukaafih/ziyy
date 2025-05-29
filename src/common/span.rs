use std::ops::{Add, AddAssign, Sub};

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

    pub fn null() -> Self {
        let pos = Position::new(0, 0);
        Span::new(pos, pos)
    }
}

impl Add<(u32, u32)> for Span {
    type Output = Span;

    fn add(mut self, rhs: (u32, u32)) -> Self::Output {
        self.end += rhs;
        self
    }
}

impl AddAssign<(u32, u32)> for Span {
    fn add_assign(&mut self, rhs: (u32, u32)) {
        self.end += rhs;
    }
}

impl Sub<(u32, u32)> for Span {
    type Output = Span;

    fn sub(mut self, rhs: (u32, u32)) -> Self::Output {
        self.start -= rhs;
        self
    }
}
