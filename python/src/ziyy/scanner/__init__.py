from .token import *

def is_alpha(c: str) -> bool:
    return len(c) > 0 and c.isascii() and c.isalpha() or c == '_'

def is_digit(c: str) -> bool:
    return len(c) > 0 and c.isascii() and c.isdigit()

class Scanner:
    def __init__(self, source: str) -> None:
        self.source = source
        self.start = 0
        self.current = 0
        self.line = 1
        self.text_line = 1
        self.text_mode = True
        self.escape = 0

    def is_at_end(self) -> bool:
        return self.current + 1 > len(self.source)

    def advance(self) -> str:
        self.current += 1
        return self.source[self.current - 1]

    def peek(self) -> str:
        try:
            c = self.source[self.current]
        except IndexError:
            return '\0'
        else:
            return c

    def peek_next(self) -> str:
        try:
            c = self.source[self.current + 1]
        except IndexError:
            return '\0'
        else:
            return c

    def make_token(self, kind: TokenKind) -> Token:
        s = self.source[self.start:self.current]
        return Token(kind, s, 0, self.line)

    def error_token(self, code: int) -> Token:
        s = self.source[self.start:self.current]
        return Token(TokenKind.Error, s, code, self.line)

    def text_token(self) -> Token:
        s = self.source[self.start:self.current]
        token = Token(TokenKind.Text, s, 0, self.line)
        self.text_line = self.line
        return token


    def skip_whitespace(self) -> None:
        if self.text_mode:
            return
        while True:
            c = self.peek()
            match c:
                case ' ' | '\r' | '\t':
                    self.advance()
                    continue
                case '\n':
                    self.line += 1
                    self.text_line += 1
                    self.advance()
                    continue
                case _:
                    return

    def check_keyword(
        self,
        start: int,
        length: int,
        rest: str,
        kind: TokenKind,
    ) -> TokenKind:
        s = self.source[(self.start + start):self.current]
        if self.current - self.start == start + length and s == rest:
            return kind
        else:
            return TokenKind.Identifier

    def identifier_kind(self) -> TokenKind:
        if self.current - self.start == 1:
            match self.source[self.start]:
                case 'b': return TokenKind.B
                case 'c': return TokenKind.C
                case 'i': return TokenKind.I
                case 's': return TokenKind.S
                case 'u': return TokenKind.U
                case 'x': return TokenKind.X
                case _: return TokenKind.Identifier
        else:
            match self.source[self.start]:
                case 'b':
                    match self.source[self.start + 1]:
                        case 'l':
                            match self.source[self.start + 2]:
                                case 'a': return self.check_keyword(3, 2, "ck", TokenKind.Black)
                                case 'u': return self.check_keyword(3, 1, "e", TokenKind.Blue)
                                case _: return TokenKind.Identifier
                        case _: return TokenKind.Identifier
                case 'c': return self.check_keyword(1, 3, "yan", TokenKind.Cyan)
                case 'g': return self.check_keyword(1, 4, "reen", TokenKind.Green)
                case 'm': return self.check_keyword(1, 6, "agenta", TokenKind.Magenta)
                case 'r':
                    match self.source[self.start + 1]:
                        case 'e': return self.check_keyword(2, 1, "d", TokenKind.Red)
                        case 'g': return self.check_keyword(2, 1, "b", TokenKind.Rgb)
                        case _: return TokenKind.Identifier
                case 'w': return self.check_keyword(1, 4, "hite", TokenKind.White)
                case 'y': return self.check_keyword(1, 5, "ellow", TokenKind.Yellow)
                case _: return TokenKind.Identifier

    def identifier(self) -> Token:
        while is_alpha(self.peek()) or is_digit(self.peek()):
            self.advance()
        kind = self.identifier_kind()
        return self.make_token(kind)

    def number(self) -> Token:
        while is_digit(self.peek()):
            self.advance()
        return self.make_token(TokenKind.Number)

    def scan_token(self) -> Token:
        if self.escape == 0:
            self.skip_whitespace()
        self.start = self.current
        if self.escape == 2 and self.peek() == '\\':
            self.escape = 1
            self.advance()
            return self.make_token(TokenKind.BackSlash)
        if self.escape == 1:
            self.escape = 0
            self.advance()
            return self.text_token()
        if self.is_at_end():
            return self.make_token(TokenKind.Eof)

        c = self.advance()
        if c == '<':
            self.text_mode = False
            return self.make_token(TokenKind.OpenTag)
        elif c == '>':
            self.text_mode = True
            return self.make_token(TokenKind.CloseTag)

        if self.text_mode:
            while not self.is_at_end():
                if self.peek() == '\n':
                    self.line += 1
                if self.peek() == '\\':
                    self.escape = 2
                    return self.text_token()
                if self.peek() != '<':
                    self.advance()
                else:
                    break
            return self.text_token()

        if is_alpha(c):
            return self.identifier()
        if is_digit(c):
            return self.number()

        match c:
            case '(': return self.make_token(TokenKind.LeftParen)
            case ')': return self.make_token(TokenKind.RightParen)
            case ',': return self.make_token(TokenKind.Comma)
            case '.': return self.make_token(TokenKind.Dot)
            case '/': return self.make_token(TokenKind.Slash)
            case _: return self.error_token(1)