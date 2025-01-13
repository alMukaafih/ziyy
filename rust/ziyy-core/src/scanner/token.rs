use super::position::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,  /* ( */
    RightParen, /* ) */
    //LeftBrace/* { */, RightBrace/* } */,
    //LeftSquare/* [ */, RightSquare/* ] */,
    //Colon/* : */,
    Equal, /* = */
    Comma, /* , */
    //SemiColon/* ; */,
    OpenTag,
    OpenTagAndSlash,
    CloseTag,
    SlashAndCloseTag,
    //Plus,
    Dot,
    Slash,

    TemplateLiteral,

    // C-Escapes
    EscA,
    EscB,
    EscT,
    EscN,
    EscV,
    EscF,
    EscR,
    EscE,

    Esc0, // Octal Escape \0XXX
    EscX, // Hex Escape \xHHH
    EscU, // Unicode Escape \uHHHH

    Escape,

    // Literals.
    Identifier,
    String,
    Number,
    WhiteSpace,
    Text,
    // Builtin Variables.
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Rgb,
    Hex, /* #ffffff */
    Byte,
    B,
    C,
    I,
    S,
    T,
    U,
    X,
    // Keywords.
    Eof,
    Error,
}

impl TokenKind {
    #[must_use] pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub content: &'a str,
    pub err_code: u8,
    pub start_pos: Position,
    pub end_pos: Position,
}
