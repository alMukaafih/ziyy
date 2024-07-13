package ziyy.compiler;

import static ziyy.value.Value.B;
import static ziyy.value.Value.I;
import static ziyy.value.Value.RESET;
import static ziyy.value.Value.RESET_B;
import static ziyy.value.Value.RESET_I;
import static ziyy.value.Value.RESET_S;
import static ziyy.value.Value.RESET_U;
import static ziyy.value.Value.S;
import static ziyy.value.Value.U;

import java.io.PrintStream;
import java.util.Map;

import ziyy.scanner.Scanner;
import ziyy.scanner.Token;
import ziyy.scanner.TokenKind;
import ziyy.value.C;
import ziyy.value.X;

public class Parser {
    Scanner scanner;
    PrintStream out;
    State state = new State();
    Map<String, String> variables;

    public Parser(String source,
            PrintStream out,
            Map<String, String> variables) {
        this.scanner = new Scanner(source);
        this.out = out;
        this.variables = variables;
    }

    public void parseToOut() throws ParseException {
        out.print(RESET);
        for (;;) {
            Token token = scanner.scanToken();
            switch (token.kind) {
                case Text:
                    out.print(token.content);
                    break;

                case B:
                    state.push(token.content, B);
                    out.print(B);

                    token = scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case I:
                    state.push(token.content, I);
                    out.print(I);

                    token = scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case S:
                    state.push(token.content, S);
                    out.print(S);

                    token = scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case U:
                    state.push(token.content, U);
                    out.print(U);

                    token = scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case C:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case Dot:
                            token = this.scanner.scanToken();
                            switch (token.kind) {
                                case Black:
                                    state.push("c", C.BLACK);
                                    out.print(C.BLACK);
                                    break;

                                case Blue:
                                    state.push("c", C.BLUE);
                                    out.print(C.BLUE);
                                    break;

                                case Cyan:
                                    state.push("c", C.CYAN);
                                    out.print(C.CYAN);
                                    break;

                                case Green:
                                    state.push("c", C.GREEN);
                                    out.print(C.GREEN);
                                    break;

                                case Magenta:
                                    state.push("c", C.MAGENTA);
                                    out.print(C.MAGENTA);
                                    break;

                                case Red:
                                    state.push("c", C.RED);
                                    out.print(C.RED);
                                    break;

                                case White:
                                    state.push("c", C.WHITE);
                                    out.print(C.WHITE);
                                    break;

                                case Yellow:
                                    state.push("c", C.YELLOW);
                                    out.print(C.YELLOW);
                                    break;

                                case Rgb:
                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.LeftParen)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int r = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Comma)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int g = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Comma)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int b = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.RightParen)
                                        throw new ParseException();

                                    String rgb = C.rgb(r, g, b);
                                    state.push("c", rgb);
                                    out.print(rgb);
                                    break;

                                default:
                                    throw new ParseException();
                            }
                            break;

                        default:
                            throw new ParseException();
                    }
                    token = this.scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;
                case X:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case Dot:
                            token = this.scanner.scanToken();
                            switch (token.kind) {
                                case Black:
                                    state.push("x", X.BLACK);
                                    out.print(X.BLACK);
                                    break;

                                case Blue:
                                    state.push("x", X.BLUE);
                                    out.print(X.BLUE);
                                    break;

                                case Cyan:
                                    state.push("x", X.CYAN);
                                    out.print(X.CYAN);
                                    break;

                                case Green:
                                    state.push("x", X.GREEN);
                                    out.print(X.GREEN);
                                    break;

                                case Magenta:
                                    state.push("x", X.MAGENTA);
                                    out.print(X.MAGENTA);
                                    break;

                                case Red:
                                    state.push("x", X.RED);
                                    out.print(X.RED);
                                    break;

                                case White:
                                    state.push("x", X.WHITE);
                                    out.print(X.WHITE);
                                    break;

                                case Yellow:
                                    state.push("x", X.YELLOW);
                                    out.print(X.YELLOW);
                                    break;

                                case Rgb:
                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.LeftParen)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int r = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Comma)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int g = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Comma)
                                        throw new ParseException();

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.Number)
                                        throw new ParseException();
                                    int b = Integer.parseInt(token.content);

                                    token = scanner.scanToken();
                                    if (token.kind != TokenKind.RightParen)
                                        throw new ParseException();

                                    String rgb = X.rgb(r, g, b);
                                    state.push("x", rgb);
                                    out.print(rgb);
                                    break;

                                default:
                                    throw new ParseException();
                            }
                            break;

                        default:
                            throw new ParseException();
                    }
                    token = this.scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case Slash:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case B:
                            if (!token.content.equals("b"))
                                throw new ParseException();
                            state.pop();
                            out.print(RESET_B);
                            break;

                        case I:
                            if (!token.content.equals("i"))
                                throw new ParseException();
                            state.pop();
                            out.print(RESET_I);
                            break;

                        case S:
                            if (!token.content.equals("s"))
                                throw new ParseException();
                            state.pop();
                            out.print(RESET_S);
                            break;

                        case U:
                            if (!token.content.equals("u"))
                                throw new ParseException();
                            state.pop();
                            out.print(RESET_U);
                            break;

                        case C: {
                            if (!token.content.equals("c"))
                                throw new ParseException();
                            state.pop();
                            String saved = state.currentSave();
                            out.print(saved);
                            break;
                        }

                        case X: {
                            if (!token.content.equals("x"))
                                throw new ParseException();
                            state.pop();
                            String saved = state.currentSave();
                            out.print(saved);
                            break;
                        }

                        case Identifier:
                        case Black:
                        case Blue:
                        case Cyan:
                        case Green:
                        case Magenta:
                        case Red:
                        case Rgb:
                        case White:
                        case Yellow: {
                            if (!state.currentTag().equals(token.content))
                                throw new ParseException();
                            state.pop();
                            String saved = state.currentSave();
                            out.print(saved);
                            break;
                        }

                        default:
                            throw new ParseException();
                    }
                    token = this.scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case Identifier:
                case Black:
                case Blue:
                case Cyan:
                case Green:
                case Magenta:
                case Red:
                case Rgb:
                case White:
                case Yellow:
                    String val = variables.get(token.content);
                    if (val != null) {
                        state.push(token.content, val);
                        out.print(val);
                    }

                    token = this.scanner.scanToken();
                    if (token.kind != TokenKind.CloseTag)
                        throw new ParseException();
                    break;

                case Error:
                    throw new ParseException("Unexpected character. " + token.content);

                case Eof:
                    out.print(RESET);
                    return;

                default:
                    continue;
            }
        }
    }
}
