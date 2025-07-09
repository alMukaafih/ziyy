use std::{
    fmt::Display,
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq)]
/// A position in source code.
pub struct Position {
    /// Line in source code.
    pub line: i32,
    /// Column in source code.
    pub column: i32,
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl Position {
    /// Creates a new Position.
    pub fn new(line: i32, column: i32) -> Self {
        Self { line, column }
    }
}

impl AddAssign<(i32, i32)> for Position {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        let (line, column) = rhs;
        self.line += line;
        if line > 0 {
            self.column = 1;
        } else {
            self.column += column;
        }
    }
}

impl SubAssign<(i32, i32)> for Position {
    fn sub_assign(&mut self, rhs: (i32, i32)) {
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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_fmt(format_args!("({},{})", self.line, self.column))
        } else {
            f.write_fmt(format_args!("{}:{}", self.line, self.column))
        }
    }
}
