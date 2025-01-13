use core::fmt::Debug;

#[derive(Clone)]
pub struct Position {
    pub line: u16,
    pub column: u16,
}

impl Position {
    #[must_use]
    pub fn new(line: u16, column: u16) -> Self {
        Self { line, column }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.column == other.column
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.line + 1, self.column + 1))?;

        Ok(())
    }
}
