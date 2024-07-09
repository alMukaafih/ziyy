from typing import TextIO
from sys import stdout
from scanner import Scanner
from scanner.token import TokenKind

def debug(source: str, out: TextIO):
    scanner = Scanner(source)
    line = -1
    while True:
        token = scanner.scan_token()
        if token.err_code == 0:
            content = token.content
        else:
            content = "Unexpected character."
        if token.line != line:
            out.write("{:4} ".format(token.line))
            line = token.line
        else:
            out.write("   | ")
        out.write("{} '{}'\n".format(token.kind, content))
        if token.kind == TokenKind.Eof:
            break

if __name__ == "__main__":
    debug("""Convenient Terminal Output Styler.

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
""", stdout)