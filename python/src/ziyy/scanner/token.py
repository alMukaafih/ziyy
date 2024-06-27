from enum import auto, Enum

class TokenKind(Enum):
    LeftParen = 0
    RightParen = auto()
    Comma = auto()
    OpenTag = auto()
    CloseTag = auto()
    Dot = auto()
    Slash = auto()
    BackSlash = auto()
    # Literals.
    Identifier = auto()
    Number = auto()
    Text = auto()
    # Builtin Variables.
    Black = auto()
    Red = auto()
    Green = auto()
    Yellow = auto()
    Blue = auto()
    Magenta = auto()
    Cyan = auto()
    White = auto()
    Rgb = auto()
    B = auto()
    C = auto()
    I = auto()
    S = auto()
    U = auto()
    X = auto()
    # Keywords.
    Eof = auto()
    Error = auto()

class Token:
    def __init__(
        self,
        kind: TokenKind,
        content: str,
        err_code: int,
        line: int
    ) -> None:
        self.kind = kind
        self.content = content
        self.err_code = err_code
        self.err_code = err_code
        self.line = line