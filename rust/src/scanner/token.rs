#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,  /* ( */
    RightParen, /* ) */
    //LeftBrace/* { */, RightBrace/* } */,
    //LeftSquare/* [ */, RightSquare/* ] */,
    //Colon/* : */,
    Equal/* = */,
    Comma, /* , */
    //SemiColon/* ; */,
    OpenTag,
    OpenTagAndSlash,
    CloseTag,
    SlashAndCloseTag,
    //Plus,
    Dot,
    Slash,
    BackSlash,
    // Literals.
    Identifier,
    String, Number,
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
    Hex/* #ffffff */,
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
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub content: &'a str,
    pub err_code: u8,
    pub line: i32,
}
