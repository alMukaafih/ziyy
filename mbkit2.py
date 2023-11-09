#!/bin/python
#################################################
# Name: mbkit.py #
# A library for styling text using escape sequence.
#
# Usage: style <option> <text> #
# Author: Tabriik # Date: 2023/08/26 
#################################################
import re
import sys
import os

class ColorError(Exception):
    pass
    #def __str__(self):
        #return "ColorError"
# Changes the text color #
# param: color instruction #
# returns: escape sequence representing color.
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
        elif color == "magenta": self.color = self.escape(5)
        elif color == "cyan": self.color = self.escape(6)
        elif color == "white": self.color = self.escape(7)
        # rgb colors
        elif color.startswith("rgb(") and color.endswith(")"):
		        self.rgb = re.findall(r"rgb\((.+)\)", color)
		        self.rgb = re.sub(r"\,", ";", self.rgb[0])
		        self.rgb = re.sub(r" ", "", self.rgb)
		        self.color = self.escape(f"8;2;{self.rgb}")
        else:
            #print(style(f"-b -c red Error: -c yellow Unknown color -x red {color}"))
            raise ColorError(style(f"-c yellow -x red {color}"))

            sys.exit(1)
            #self.color = ""

        color = re.sub(r"\(", r"\(", color, count=1)
        color = re.sub(r"\)", r"\)", color, count=1)
        self.sub = color
    def substitute(self, *options):
        global string
        for option in options:
            string = re.sub(f"{option}\s{self.sub}\s", self.color, string, count=1)


# initialize fg and bg
fg = Color(3)
bg = Color(4)

def style(argument):
	# VARIABLES
	global reset, string, fgcolor
	reset = "\x1b[0m" # reset Creset
	string = argument       # reset string
	fgcolor = ""
	bgcolor = ""
	args = argument.split()
	x = 1             # index of list args
	while x <= len(args):
			
		# arguments
		y = x - 1
		arg1 = args[y]
		if 0 <= x < len(args):
			arg2 = args[x]  
		
		
		#foreground colors
		if arg1 == "--fg" or arg1 == "-c":
			
			# match the end of a bracket
			while "(" in arg2 and ")" not in arg2:
				fsave = arg2
				args = args[1:]
				arg2 = f"{fsave} {args[1]}"
			fg.colorValue(arg2)
			fg.substitute("--fg", "-c")
			args = args[2:]
		
		# background colors
		elif arg1 == "--bg" or arg1 == "-x":
			# match the end of a bracket
			while "(" in arg2 and ")" not in arg2:
				bsave = arg2
				args = args[1:]
				arg2 = f"{bsave} {args[1]}"
			bg.colorValue(arg2)
			bg.substitute("--bg", "-x")
			args = args[2:]
		
		# Bold
		elif arg1 == "-b":
			string = re.sub(f"-b ", "\x1b[1m", string, count=1)
			args = args[1:]
		# Remove Bold
		elif arg1 == "-rb":
			string = re.sub(r"\s-rb", "\x1b[21m", string, count=1)
			args = args[1:]
		
		# Italics
		elif arg1 == "-i":
			string = re.sub(r"-i\s", "\x1b[3m", string, count=1)
			args = args[1:]
		# Remove italics
		elif arg1 == "-ri":
			string = re.sub(r"\s-ri","\x1b[23m" , string, count=1)
			args = args[1:]
		
		# Remove colors
		elif arg1 == "-rc":
			string = re.sub(r"\s-rc","\x1b[39m" , string, count=1)
			args = args[1:]
		elif arg1 == "-rx":
			string = re.sub(r"\s-rx","\x1b[49m" , string, count=1)
			args = args[1:]
			
		# Underline
		elif arg1 == "-u":
			string = re.sub(r"-u\s","\x1b[4m" , string, count=1)
			args = args[1:]
		elif arg1 == "-ru":
			string = re.sub(r"\s-ru","\x1b[24m" , string, count=1)
			args = args[1:]
			
		# Strike through
		elif arg1 == "-s":
			string = re.sub(r"-s\s","\x1b[9m" , string, count=1)
			args = args[1:]
		elif arg1 == "-rs":
			string = re.sub(r"\s-rs","\x1b[29m" , string, count=1)
			args = args[1:]
			
		# Single Quotes
		elif arg1 == "-q":
			save = fgcolor
			__fg_color(arg2)
			chose = fgcolor
			quotes = f"'{chose}"
			string = re.sub(f"-q {arg2}", quotes, string, count=1)
			args = args[2:]
		elif arg1 == "-rq":
			quotes = f"{save}'"
			string = re.sub(f" -rq", quotes, string, count=1)
			args = args[1:]
		
		# Double Quotes
		elif arg1 == "-Q":
			save = fgcolor
			__fg_color(arg2)
			chose = fgcolor
			quotes = f"\"{chose}"
			string = re.sub(f"-Q {arg2}", quotes, string, count=1)
			args = args[2:]
		elif arg1 == "-rQ":
			quotes = f"{save}\""
			string = re.sub(f" -rQ", quotes, string, count=1)
			args = args[1:]
		
		# Unknown Options
		elif arg1.startswith("-"):
			string = re.sub(r"-.*?\s", "", string, count=1)
			args = args[1:]
			
		# Strings
		else:
			args = args[1:]
		
		arg1 = ""
		arg2 = ""
	return f"{string}{reset}"

class Tmplt:
    def __init__(self, save):
        self.save = save
    def style(self, string):
        return style(f"{self.save} {string}")

if __name__ == "__main__":
    print(style("-x rrgb(0,150,75) -b -c rgb(255, 255, 255)  Yes  -rc -x rgb(150,0,0)  No! "))
    print(style("-b -c rgb(0, 125, 250) Hello World!"))
    print(style("-b -u Hello World!"))
    print(style("-x yllow  -u -c red Hello -ru --fg blue World! -c red Now "))


