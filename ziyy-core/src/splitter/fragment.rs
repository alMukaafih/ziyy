use crate::common::Span;

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
    pub span: Span,
}

impl Fragment {
    pub fn new(r#type: FragmentType, lexeme: String, span: Span) -> Self {
        Fragment {
            r#type,
            lexeme,
            span,
        }
    }
}
