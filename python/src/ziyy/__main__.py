import sys
import os

from .__init__ import style
def usage():
    return style("""Convenient Terminal Output Styler.

[c:green][b]Usage: [c:cyan]ziyy[/b] [c:cyan][OPTION] [TEXT]

[b][c:green]Options:[/0]
  [c:cyan][b]-V[/0], [c:cyan][b]--version[/0]
          Print version info and exit
  [c:cyan][b]-f[/0], [c:cyan][b]--file[/b] <FILENAME>[/c]
          Read input from file.
  [c:cyan][b]-n[/0], [c:cyan][b]--no-newline[/0]
          Do not print newline after text.
  [c:cyan][b]-h[/0], [c:cyan][b]--help[/0]
          Print help
""")
    
args = sys.argv[1:]
if len(args) < 1:
    print(usage(), end="")
    sys.exit(1)
first = args[0]
if first == "-n" or first == "--no-newline":
    print(style(args[1]), end="")
elif first == "-f" or first == "--file":
    if len(args) == 1:
        sys.exit(1)
    if not os.path.isfile(args[1]):
        sys.exit(1)
    file = open(args[1], "r").read()
    print(style(file), end="")
elif first == "-V" or first == "--version":
    print("ziyy 1.0.0")
elif first == "-h" or first == "--help":
    print(usage(), end="")
    sys.exit(0)
else:
    print(style(first))