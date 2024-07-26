from .parser import Parser
from typing import TextIO

class Compiler:
    def __init__(self, source: str, out: TextIO, variables: dict[str, str]):
        self.parser = Parser(source, out, variables)

    def compile(self):
        self.parser.parse_to_out()

    def compile_source(self, source: str):
        self.parser.scanner.source = source
        self.compile()