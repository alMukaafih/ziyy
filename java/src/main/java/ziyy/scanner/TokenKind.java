package ziyy.scanner;

public enum TokenKind {
     // Single-character tokens.
    LeftParen,  /* ( */
    RightParen, /* ) */
    //LeftBrace/* { */, RightBrace/* } */,
    //LeftSquare/* [ */, RightSquare/* ] */,
    //Colon/* : */, Equal/* = */,
    Comma, /* , */
    //SemiColon/* ; */,
    OpenTag,
    CloseTag,
    //Plus,
    Dot,
    Slash,
    BackSlash,
    // Literals.
    Identifier,
    /* String, */ Number,
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
    B,
    C,
    I,
    S,
    U,
    X,
    // Keywords.
    Eof,
    Error,
}