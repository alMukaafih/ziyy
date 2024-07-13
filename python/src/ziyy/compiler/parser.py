from .state import State
from ..scanner.token import TokenKind as TT
from ..scanner import Scanner
from ..value import *
from typing import TextIO

class ParseException(Exception):
    pass

class Parser:
    def __init__(self, source: str, out: TextIO, variables: dict[str, str]):
        self.scanner = Scanner(source)
        self.out = out
        self.state = State()
        self.variables = variables

    def parse_to_out(self):
        self.out.write(RESET)
        while True:
            token = self.scanner.scan_token()
            match token.kind:
                case TT.Text:
                    self.out.write(token.content)
                case TT.B:
                    self.state.push(token.content, B)
                    self.out.write(B)

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.I:
                    self.state.push(token.content, I)
                    self.out.write(I)

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.S:
                    self.state.push(token.content, S)
                    self.out.write(S)

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.U:
                    self.state.push(token.content, U)
                    self.out.write(U)

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.C:
                    token = self.scanner.scan_token()
                    match token.kind:
                        case TT.Dot:
                            token = self.scanner.scan_token()
                            match token.kind:
                                case TT.Black:
                                    self.state.push("c", C.BLACK)
                                    self.out.write(C.BLACK)
                                case TT.Blue:
                                    self.state.push("c", C.BLUE)
                                    self.out.write(C.BLUE)
                                case TT.Cyan:
                                    self.state.push("c", C.CYAN)
                                    self.out.write(C.CYAN)
                                case TT.Green:
                                    self.state.push("c", C.GREEN)
                                    self.out.write(C.GREEN)
                                case TT.Magenta:
                                    self.state.push("c", C.MAGENTA)
                                    self.out.write(C.MAGENTA)
                                case TT.Red:
                                    self.state.push("c", C.RED)
                                    self.out.write(C.RED)
                                case TT.White:
                                    self.state.push("c", C.WHITE)
                                    self.out.write(C.WHITE)
                                case TT.Yellow:
                                    self.state.push("c", C.YELLOW)
                                    self.out.write(C.YELLOW)
                                case TT.Rgb:
                                    token = self.scanner.scan_token()
                                    if token.kind != TT.LeftParen:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    r: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Comma:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    g: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Comma:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    b: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.RightParen:
                                        raise ParseException

                                    rgb = C.rgb(r, g, b)
                                    self.state.push("c", rgb)
                                    self.out.write(rgb)
                                case _: raise ParseException
                        case _: raise ParseException

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.X:
                    token = self.scanner.scan_token()
                    match token.kind:
                        case TT.Dot:
                            token = self.scanner.scan_token()
                            match token.kind:
                                case TT.Black:
                                    self.state.push("x", X.BLACK)
                                    self.out.write(X.BLACK)
                                case TT.Blue:
                                    self.state.push("x", X.BLUE)
                                    self.out.write(X.BLUE)
                                case TT.Cyan:
                                    self.state.push("x", X.CYAN)
                                    self.out.write(C.CYAN)
                                case TT.Green:
                                    self.state.push("x", X.GREEN)
                                    self.out.write(X.GREEN)
                                case TT.Magenta:
                                    self.state.push("x", X.MAGENTA)
                                    self.out.write(C.MAGENTA)
                                case TT.Red:
                                    self.state.push("x", X.RED)
                                    self.out.write(C.RED)
                                case TT.White:
                                    self.state.push("x", X.WHITE)
                                    self.out.write(X.WHITE)
                                case TT.Yellow:
                                    self.state.push("x", X.YELLOW)
                                    self.out.write(X.YELLOW)
                                case TT.Rgb:
                                    token = self.scanner.scan_token()
                                    if token.kind != TT.LeftParen:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    r: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Comma:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    g: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Comma:
                                        raise ParseException

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.Number:
                                        raise ParseException
                                    b: int = int(token.content)

                                    token = self.scanner.scan_token()
                                    if token.kind != TT.RightParen:
                                        raise ParseException

                                    rgb = X.rgb(r, g, b)
                                    self.state.push("x", rgb)
                                    self.out.write(rgb)
                                case _: raise ParseException
                        case _: raise ParseException

                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.Slash:
                    token = self.scanner.scan_token()
                    match token.kind:
                        case TT.B:
                            if self.state.current_tag() != "b":
                                raise ParseException
                            self.state.pop()
                            self.out.write(RESET_B)
                        case TT.I:
                            if self.state.current_tag() != "i":
                                raise ParseException
                            self.state.pop()
                            self.out.write(RESET_I)
                        case TT.S:
                            if self.state.current_tag() != "s":
                                raise ParseException
                            self.state.pop()
                            self.out.write(RESET_S)
                        case TT.U:
                            if self.state.current_tag() != "u":
                                raise ParseException
                            self.state.pop()
                            self.out.write(RESET_U)
                        case TT.C:
                            if self.state.current_tag() != "c":
                                raise ParseException
                            self.state.pop()
                            saved = self.state.current_save()
                            self.out.write(saved)
                        case TT.X:
                            if self.state.current_tag() != "x":
                                raise ParseException
                            self.state.pop()
                            saved = self.state.current_save()
                            self.out.write(saved)
                        case TT.Identifier | TT.Black | TT.Blue | TT.Cyan | TT.Green | TT.Magenta | TT.Red | TT.Rgb | TT.White | TT.Yellow:
                            if self.state.current_tag() != token.content:
                                raise ParseException
                            self.state.pop()
                            saved = self.state.current_save()
                            self.out.write(saved)
                        case _: raise ParseException
                    token = self.scanner.scan_token()
                    if token.kind != TT.CloseTag:
                        raise ParseException
                case TT.Identifier | TT.Black | TT.Blue | TT.Cyan | TT.Green | TT.Magenta | TT.Red | TT.Rgb | TT.White | TT.Yellow:
                    var = self.variables.get(token.content)
                    if var is not None:
                        self.state.push(token.content, var)
                        self.out.write(var)
                    else:
                        raise ParseException

                case TT.Eof:
                    self.out.write(RESET)
                    break
                case _: continue