from .state import State
from ..scanner.token import TokenKind as TT
from ..scanner import Scanner
from ..value import *
from typing import TextIO

class Parser:
    def __init__(self, source: str, out: TextIO, variables: dict[str, str]):
        self.scanner = Scanner(source)
        self.out = out
        self.state = State()
        self.variables = variables

    def parse_to_out(self):
        self.out.write(RESET)
        token = self.scanner.scan_token()