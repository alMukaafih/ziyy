import re
import sys
import os

from typing import TextIO
from .value import C
from .compiler import Compiler

__all__ = ["style", "template", "Compiler"]

class String(TextIO):
    def __init__(self):
        self.s = ""

    def write(self, s: str) -> int:
        self.s = self.s + s
        return len(s)

def _compile(source: str, out: TextIO):
    vars = {
        "green": C.rgb(0, 150, 75),
        "cyan": C.rgb(0, 150, 150)
    }
    compiler = Compiler(source, out, vars)
    compiler.compile()


def style(text: str) -> str:
    vars: dict[str, str] = {}
    out = String()
    compiler = Compiler(text, out, vars)
    compiler.compile()
    return out.s

def template(save: str):
    return lambda text: style(f"{save}{text}")
