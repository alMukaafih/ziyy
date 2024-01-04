#!/bin/python
import re

class Color:
    def __init__(self, firstDigit):
        self.firstDigit = firstDigit

    def escape(self, secondDigit):
        self.secondDigit = secondDigit
        return f"\x1b[{self.firstDigit}{self.secondDigit}m"

    def colorValue(self, color):
        global string
        # basic colors
        if color == "black": self.color = self.escape(0)
        elif color == "red": self.color = self.escape(1)
        elif color == "green": self.color = self.escape(2)
        elif color == "yellow": self.color = self.escape(3)
        elif color == "blue": self.color = self.escape(4)
        elif color == "magenta" self.color = self.escape(5)
        elif color == "cyan": self.color = self.escape(6)
        elif color == "white": self.color = self.escape(7)
        # rgb colors
        elif color.startswith("rgb(") and color.endswith(")"):
		    self.rgb = re.findall(r"rgb\((.+)\)", color)
		    self.rgb = re.sub(r"\,", ";", self.rgb[0])
		    self.rgb = re.sub(r" ", "", self.rgb)
		    self.color = self.escape(f"8;2;{self.rgb}")
        else: self.color = ""
        color = re.sub(r"\(", r"\(", fgc, count=1)
	      color = re.sub(r"\)", r"\)", fgc, count=1)
        string = re.sub(f"-c\s{color}\s", self.color, string, count=1)
        string = re.sub(f"--fg\s{color}\s", self.color, string, count=1)

class Style:
    def __init__(self, argument): pass


if __name__ == "__main__":
    fg = Color(3)
    bg = Color(4)
    print(f"{fg.colorValue('red')"
