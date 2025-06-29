use crate::common::Span;

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    FG_BLACK,
    FG_RED,
    FG_GREEN,
    FG_YELLOW,
    FG_BLUE,
    FG_MAGENTA,
    FG_CYAN,
    FG_WHITE,
    FG_RGB,
    FG_HEX,
    FG_FIXED,
    FG_DEFAULT,

    BG_BLACK,
    BG_RED,
    BG_GREEN,
    BG_YELLOW,
    BG_BLUE,
    BG_MAGENTA,
    BG_CYAN,
    BG_WHITE,
    BG_RGB,
    BG_HEX,
    BG_FIXED,
    BG_DEFAULT,

    LEFT_PAREN,
    RIGHT_PAREN,
    PLACE_HOLDER,
    COMMA,
    NUMBER,
    IDENTIFIER,
}

#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub literal: Option<u8>,
    pub span: Span,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, literal: Option<u8>, span: Span) -> Self {
        Token {
            r#type,
            literal,
            lexeme,
            span,
        }
    }
}
