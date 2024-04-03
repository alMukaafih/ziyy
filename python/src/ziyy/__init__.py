#!/bin/python
import re
import sys
import os

class ColorError(Exception):
    pass
    #def __str__(self):
        #return "ColorError"
class Color:
    def __init__(self, firstDigit):
        self.firstDigit = firstDigit

    def escape(self, secondDigit):
        self.secondDigit = secondDigit
        return f"\x1b[{self.firstDigit}{self.secondDigit}m"

    def color_value(self, color):
        global string
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

        color = re.sub(r"\(", r"\(", color, count=1)
        color = re.sub(r"\)", r"\)", color, count=1)
        self.sub = color
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

def style(text):
    # initialize fg and bg
    fg = Color(3)
    bg = Color(4)
    RESET = "\x1b[0m" # RESET

    p = Parser()
    p.parse(text)
    text = p.result
    for tag in p.tags:
        if tag.startswith("[c:"):
            value = tag[3:-1]
            fg.color_value(value)
            text = fg.substitute(text, tag)
        elif tag.startswith("[x:"):
            value = tag[3:-1]
            bg.color_value(value)
            text = bg.substitute(text, tag)
        # Bold
        if "[b]" in text:
            text = text.replace("[b]", "\x1b[1m")
        # Remove Bold
        if "[/b]" in text:
            text = text.replace("[/b]", "\x1b[22m")
        # Italics
        if "[i]" in text:
            text = text.replace("[i]", "\x1b[3m")
            
        # Remove italics
        if "[/i]" in text: 
            text = text.replace("[/i]", "\x1b[23m")
        
        
        # Remove colors
        if "[/c]" in text:
            text = text.replace("[/c]", "\x1b[39m")
        
        if "[/x]" in text:
            text = text.replace("[/x]", "\x1b[49m")
        
        # Underline
        if "[u]" in text:
            text = text.replace("[u]", "\x1b[4m")
        
        if "[/u]" in text:
            text = text.replace("[/u]", "\x1b[24m")
        
    
        # Strike through
        if "[s]" in text:
            text = text.replace("[s]", "\x1b[9m")
    
        
        if "[/s]" in text:
            text = text.replace("[/s]/", "\x1b[29m")
        
        if "[/0]" in text:
            text = text.replace("[/0]", RESET)
            

    return f"{text}{RESET}"
    

def template(save):
        return lambda text: style(f"{save}{text}")

if __name__ == "__main__":
    p = template("[b][c:blue]")
    print(p("Yes"))
    print(p("Hello"))
