use crate::common::Span;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Default)]
pub enum TokenType {
    ESCAPE,
    ESCAPE_A,
    ESCAPE_B,
    ESCAPE_E,
    ESCAPE_F,
    ESCAPE_N,
    ESCAPE_R,
    ESCAPE_T,
    ESCAPE_V,
    ESCAPE_0,
    ESCAPE_X,
    ESCAPE_U,
    ESCAPE_LESS,
    ESCAPE_GREATER,
    ESCAPE_BACKSLASH,
    #[default]
    TEXT,
    EOF,
}

#[derive(Default)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub span: Span,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, span: Span) -> Self {
        Token {
            r#type,
            lexeme,
            span,
        }
    }
}
