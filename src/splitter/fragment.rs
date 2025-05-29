#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum FragmentType {
    Error,
    Tag,
    Whitespace,
    Word,
}

#[derive(Debug, Clone)]
pub struct Fragment {
    pub r#type: FragmentType,
    pub lexeme: String,
    pub line: usize,
}

impl Fragment {
    pub fn new(r#type: FragmentType, lexeme: String, line: usize) -> Self {
        Fragment {
            r#type,
            lexeme,
            line,
        }
    }
}
