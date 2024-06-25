#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen/* ( */, RightParen/* ) */,
    //LeftBrace/* { */, RightBrace/* } */,
    //LeftSquare/* [ */, RightSquare/* ] */,
    //Colon/* : */, Equal/* = */,
    Comma/* , */, //SemiColon/* ; */,
    OpenTag, CloseTag,
    //Plus,
    Dot, Slash,
    // Literals.
    Identifier, /* String, */ Number, Text,
    // Builtin Variables.
    Black, Red, Green, Yellow,
    Blue, Magenta, Cyan, White,
    B, C, I, S, U, X,
    // Keywords.
    Eof, Error
}

impl TokenKind {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub content: String,
    pub line: i32,
}