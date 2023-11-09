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

# Changes the foreground color #
# param: color instruction #
# returns: escape sequence representing color.
def __fg_color(fgc):
	global fgcolor, string
	# Basic colorsm
	if fgc == "black": fgcolor = "\x1b[30m"
	elif fgc == "red": fgcolor = "\x1b[31m"
	elif fgc == "green": fgcolor = "\x1b[32m"
	elif fgc == "yellow": fgcolor = "\x1b[33m"
	elif fgc == "blue": fgcolor = "\x1b[34m"
	elif fgc == "magenta": fgcolor = "\x1b[35m"
	elif fgc == "cyan": fgcolor = "\x1b[36m"
	elif fgc == "white": fgcolor = "\x1b[37m"
	
	# RGB Colors
	elif fgc.startswith("rgb(") and fgc.endswith(")"):
		frgb = re.findall(r"rgb\((.+)\)", fgc)
		frgb = re.sub(r"\,", ";", frgb[0])
		frgb = re.sub(r" ", "", frgb)
		fgcolor = f"\x1b[38;2;{frgb}m"
		
		
	else: fgcolor = ""
	fgc = re.sub(r"\(", r"\(", fgc, count=1)
	fgc = re.sub(r"\)", r"\)", fgc, count=1)
	string = re.sub(f"-c\s{fgc}\s", fgcolor, string, count=1)
	string = re.sub(f"--fg\s{fgc}\s", fgcolor, string, count=1)

# Changes the background color #
# param: color instruction #
# returns: escape sequence representing color.
def __bg_color(bgc):
	global bgcolor, string
	# Basic colors
	if bgc == "black": bgcolor = "\x1b[40m"
	elif bgc == "red": bgcolor = "\x1b[41m"
	elif bgc == "green": bgcolor = "\x1b[42m"
	elif bgc == "yellow": bgcolor = "\x1b[43m"
	elif bgc == "blue": bgcolor = "\x1b[44m"
	elif bgc == "magenta": bgcolor = "\x1b[45m"
	elif bgc == "cyan": bgcolor = "\x1b[46m"
	elif bgc == "white": bgcolor = "\x1b[47m"
	else: bgcolor = ""
	string = re.sub(f"-x\s{bgc}\s", bgcolor, string, count=1)
	string = re.sub(f"--bg\s{bgc}\s", bgcolor, string, count=1)

def style(argument):
	# VARIABLES
	global reset, string, n, fgcolor, styl
	reset = "\x1b[0m" # reset Creset
	string = argument       # reset string
	n = "\n"          # newline chfgcolor
	fgcolor = ""
	bgcolor = ""
	args = argument.split()
	x = 1             # index of list args
	p = 1             # spacing
	styl = ""
	while x <= len(args):
		
		# space if not at the beginning
		if p == 1:
			s = ""
		else:
			#s = f"\x1b[39m {fgcolor}"
			s = " "
			
		# arguments
		y = x - 1
		z = x + 1
		arg1 = args[y]
		if 0 <= x < len(args):
			arg2 = args[x]
		if 0 <= z < len(args):
			arg3 = args[z]
		
		
		#foreground colors
		if arg1 == "--fg" or arg1 == "-c":
			
			# match the end of a bracket
			while arg2.find("(") >= 0 and arg2.find(")") < 0:
				save = arg2
				args = args[1:]
				arg2 = f"{save} {args[1]}"
			__fg_color(arg2)
			args = args[2:]
		
		# background colors
		elif arg1 == "--bg" or arg1 == "-x":
			__bg_color(arg2)
			args = args[2:]
		
		# Bold
		elif arg1 == "-b":
			string = re.sub(f"-b ", "\x1b[1m", string, count=1)
			args = args[1:]
		# Remove Bold
		elif arg1 == "-rmb":
			string = re.sub(r"\s-rmb", "\x1b[21m", string, count=1)
			args = args[1:]
		
		# Italics
		elif arg1 == "-i":
			string = re.sub(r"-i\s", "\x1b[3m", string, count=1)
			args = args[1:]
		# Remove italics
		elif arg1 == "-rmi":
			string = re.sub(r"\s-rmi","\x1b[23m" , string, count=1)
			args = args[1:]
		
		# Remove colors
		elif arg1 == "-rmc":
			string = f"{string}\x1b[39m"
			args = args[1:]
		elif arg1 == "-rmx":
			string = f"{string}\x1b[49m"
			args = args[1:]
			
		# Underline
		elif arg1 == "-u":
			string = re.sub(r"-u\s","\x1b[4m" , string, count=1)
			args = args[1:]
		elif arg1 == "-rmu":
			string = re.sub(r"\s-rmu","\x1b[24m" , string, count=1)
			string = f"{string}\x1b[24m"
			args = args[1:]
			
		# Strike through
		elif arg1 == "-s":
			string = re.sub(r"-s\s","\x1b[9m" , string, count=1)
			args = args[1:]
		elif arg1 == "-rms":
			string = re.sub(r"\s-rms","\x1b[29m" , string, count=1)
			args = args[1:]
			
		# Quote
		elif arg1 == "-q":
			save = fgcolor
			__fg_color(arg2)
			chose = fgcolor
			quotes = f"{save} ' {chose} {arg3} {save} '"
			quotes = re.sub(r'\s', '', quotes)
			string = f"{string}{s}{quotes}"
			args = args[3:]
			p += 1
		elif arg1 == "-Q":
			save = fgcolor
			__fg_color(arg2)
			chose = fgcolor
			quotes = f"{save} \" {chose} {arg3} {save} \""
			quotes = re.sub(r'\s', '', quotes)
			string = f"{string}{s}{quotes}"
			args = args[3:]
			p += 1
		# no newline
		elif arg1 == "-n": n=""; args[1:]
		
		# Unknown Options
		elif arg1.startswith("-"):
			args = args[1:]
		# Strings
		else:
			#string= f"{string}{s}{styl}{arg1}"
			#styl = ""
			args = args[1:]
		#	p += 1
		
		arg1 = ""
		arg2 = ""
		arg3 = ""
	#print(f"{string}{reset}", end=n)
	#sys.stdout.write(f"{string}{reset}{n}")
	string = re.sub(r"-.*?\s", "", string)
	return f"{string}{reset}"




#print(style("-b -s -c red -h Hello -rms -u -c blue World! -rmu green My"))
#print(style("-b -c rgb(0, 125, 250) Hello World!"))
#print(style("-b -u Hello World!"))
#print(style("-x yellow  -u -c red Hello -rmu --fg blue World! -c red Now "))