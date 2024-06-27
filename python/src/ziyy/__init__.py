import re
import sys
import os

from typing import TextIO
from .value import C
from .compiler import Compiler

__all__ = ["style", "template"]

class String(TextIO):
    def __init__(self):
        self.s = ""

    def write(self, s: str) -> int:
        self.s = self.s + s
        return len(s)

class ColorError(Exception):
    pass

class Color:
    def __init__(self, first_digit):
        self.first_digit = first_digit

    def escape(self, second_digit):
        self.second_digit = second_digit
        return f"\x1b[{self.first_digit}{self.second_digit}m"

    def color_value(self, color):
        # basic colors
        if color == "black":
            self.color = self.escape(0)
        elif color == "red":
            self.color = self.escape(1)
        elif color == "green":
            self.color = self.escape(2)
        elif color == "yellow":
            self.color = self.escape(3)
        elif color == "blue":
            self.color = self.escape(4)
        elif color == "magenta":
            self.color = self.escape(5)
        elif color == "cyan":
            self.color = self.escape(6)
        elif color == "white":
            self.color = self.escape(7)
        # rgb colors
        elif color.startswith("rgb(") and color.endswith(")"):
            self.rgb = color[4:-1]
            self.rgb = re.sub(r"\,", ";", self.rgb)
            self.color = self.escape(f"8;2;{self.rgb}")
        else:
            raise ColorError(style(f"[c: yellow][x: red]{color}"))
    def substitute(self, text, tag):
        return text.replace(tag, self.color)

class Parser:
    def __init__(self):
        self.result = ""
        self.tags = []
        self.on = False
    def parse(self, text: str):
        chars = list(text)
        i = 0
        tag = ""
        while i < len(chars):
            if chars[i] == '\\':
                self.result += chars[i + 1]
                i += 2
                continue
            if chars[i] == '[':
                self.on = True
                self.result += chars[i]
                tag +=chars[i]
                i += 1
                continue
            if chars[i] == ']':
                self.on = False
                self.result += chars[i]
                tag += chars[i]
                if tag not in self.tags:
                    self.tags.append(tag)
                tag = ""
                i += 1
                continue
            if self.on and not chars[i].isspace():
                self.result += chars[i]
                tag += chars[i]
                i += 1
                continue
            elif self.on and chars[i].isspace():
                i += 1
                continue
            self.result += chars[i]
            i += 1

def _compile(source: str, out: TextIO):
    vars = {
        "green": C.rgb(0, 150, 75),
        "cyan": C.rgb(0, 150, 150)
    }
    compiler = Compiler(source, out, vars)
    compiler.compile()


def style(text: str) -> str:
    vars = {}
    out = String()
    compiler = Compiler(text, out, vars)
    compiler.compile()
    return out.s

def template(save):
    return lambda text: style(f"{save}{text}")

if __name__ == "__main__":
    p = template("[b][c:rgb(0,150,75)]")
    print(p("Yes"))
    print(p("Hello"))
