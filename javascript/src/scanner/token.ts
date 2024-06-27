export enum TokenKind {
    // Single-character tokens.
    LeftParen = 0,  /* ( */
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

export class Token {
    kind: TokenKind
    content: string
    err_code: number
    line: number

    constructor(
        kind: TokenKind,
        content: string,
        err_code: number,
        line: number
    ) {
        this.kind = kind
        this.content = content
        this.err_code = err_code
        this.line = line
    }
}