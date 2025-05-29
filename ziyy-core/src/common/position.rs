use std::ops::{AddAssign, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    line: u32,
    column: u32,
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

impl AddAssign<(u32, u32)> for Position {
    fn add_assign(&mut self, rhs: (u32, u32)) {
        let (line, column) = rhs;
        self.line += line;
        if line > 0 {
            self.column = 1;
        } else {
            self.column += column;
        }
    }
}

impl SubAssign<(u32, u32)> for Position {
    fn sub_assign(&mut self, rhs: (u32, u32)) {
        let (line, column) = rhs;
        self.line -= line;
        self.column -= column;
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.line.partial_cmp(&other.line) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.column.partial_cmp(&other.column)
    }
}
