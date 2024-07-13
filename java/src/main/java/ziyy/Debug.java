package ziyy;

import java.io.PrintStream;

import ziyy.scanner.Scanner;
import ziyy.scanner.Token;
import ziyy.scanner.TokenKind;

public class Debug {
    static void debug(String source, PrintStream out) {
        Scanner scanner = new Scanner(source);
        int line = -1;
        for (;;) {
            Token token = scanner.scanToken();
            String content;
            if (token.errCode == 0) {
                content = token.content;
            } else {
                content = "Unexpected character. " + token.content;
            }
            if (token.line != line) {
                out.printf("%4d ", token.line);
                line = token.line;
            } else {
                out.printf("   | ");
            }
            out.printf("%s '%s'\n", token.kind, content);
            if (token.kind == TokenKind.Eof)
                break;
        }
    }
    public static void main(String[] args) {
        debug("""
Convenient Terminal Output Styler.

<green><b><u>Usage:</u></b> <cyan><b>ziyy</b> <i>[OPTION] [TEXT]</i></cyan>

<b><u>Options:</u></b></green>
  <cyan><b>-V</b></cyan>, <cyan><b>--version</b></cyan>
          Print version info and exit
  <cyan><b>-f</b></cyan>, <cyan><b>--file</b> \\<FILENAME\\></cyan>
          Read input from file.
  <cyan><b>-n</b></cyan>, <cyan><b>--no-newline</b></cyan>
          Do not print newline after text.
  <cyan><b>-h</b></cyan>, <cyan><b>--help</b></cyan>
          Print help
""", System.out);
    }
}
