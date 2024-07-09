import sys
import os

from .__init__ import _compile

out = sys.stdout

def usage():
    _compile("""Convenient Terminal Output Styler.

<green><b><u>Usage:</u></b> <cyan><b>ziyy</b> <i>[OPTION] [TEXT]</i></cyan>

<b><u>Options:</u></b></green>
  <cyan><b>-V</b></cyan>, <cyan><b>--version</b></cyan>
          Print version info and exit
  <cyan><b>-f</b></cyan>, <cyan><b>--file</b> \<FILENAME\></cyan>
          Read input from file.
  <cyan><b>-n</b></cyan>, <cyan><b>--no-newline</b></cyan>
          Do not print newline after text.
  <cyan><b>-h</b></cyan>, <cyan><b>--help</b></cyan>
          Print help
""", out)

args = sys.argv[1:]
if len(args) < 1:
    usage()
    sys.exit(0)
first = args[0]
if first == "-n" or first == "--no-newline":
    _compile(args[1], out)
elif first == "-f" or first == "--file":
    if len(args) == 1:
        sys.exit(1)
    if not os.path.isfile(args[1]):
        sys.exit(1)
    file = open(args[1], "r").read()
    _compile(file, out)
elif first == "-V" or first == "--version":
    print("ziyy 1.0.6")
elif first == "-h" or first == "--help":
    usage()
    sys.exit(0)
else:
    _compile(first, out)