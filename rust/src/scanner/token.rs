#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen/* ( */, RightParen/* ) */,
    //LeftBrace/* { */, RightBrace/* } */,
    //LeftSquare/* [ */, RightSquare/* ] */,
    //Colon/* : */, Equal/* = */,
    Comma/* , */, //SemiColon/* ; */,
    OpenTag, CloseTag,
    //Plus,
    Dot, Slash, BackSlash,
    // Literals.
    Identifier, /* String, */ Number, Text,
    // Builtin Variables.
    Black, Red, Green, Yellow,
    Blue, Magenta, Cyan, White,
    B, C, I, S, U, X,
    // Keywords.
    Eof, Error,
    #[default]
    Def
}

impl TokenKind {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Default)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub content: &'a str,
    pub err_code: u8,
    pub line: i32,
}