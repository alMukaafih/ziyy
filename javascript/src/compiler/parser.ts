import { State } from "./state";
import { TokenKind as TT } from "../scanner/token";
import { Scanner } from "../scanner";
import {
    B,
    C,
    I,
    RESET,
    RESET_B,
    RESET_I,
    RESET_S,
    RESET_U,
    S,
    U,
    X,
} from "../value";

class ParseError extends Error {}

export class Parser {
    scanner: Scanner;
    out: NodeJS.WriteStream;
    state: State = new State();
    variables: Record<string, string>;

    constructor(
        source: string,
        out: NodeJS.WriteStream,
        variables: Record<string, string>
    ) {
        this.scanner = new Scanner(source);
        this.out = out;
        this.variables = variables;
    }

    parseToOut() {
        this.out.write(RESET);
        for (;;) {
            let token = this.scanner.scanToken();
            switch (token.kind) {
                case TT.Text:
                    this.out.write(token.content);
                    break;
                case TT.B:
                    this.state.push(token.content, B);
                    this.out.write(B);

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.I: {
                    this.state.push(token.content, I);
                    this.out.write(I);

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                }
                case TT.S:
                    this.state.push(token.content, S);
                    this.out.write(S);

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.U:
                    this.state.push(token.content, U);
                    this.out.write(U);

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.C:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case TT.Dot:
                            token = this.scanner.scanToken();
                            switch (token.kind) {
                                case TT.Black:
                                    this.state.push("c", C.BLACK);
                                    this.out.write(C.BLACK);
                                    break;
                                case TT.Blue:
                                    this.state.push("c", C.BLUE);
                                    this.out.write(C.BLUE);
                                    break;
                                case TT.Cyan:
                                    this.state.push("c", C.CYAN);
                                    this.out.write(C.CYAN);
                                    break;
                                case TT.Green:
                                    this.state.push("c", C.GREEN);
                                    this.out.write(C.GREEN);
                                    break;
                                case TT.Magenta:
                                    this.state.push("c", C.MAGENTA);
                                    this.out.write(C.MAGENTA);
                                    break;
                                case TT.Red:
                                    this.state.push("c", C.RED);
                                    this.out.write(C.RED);
                                    break;
                                case TT.White:
                                    this.state.push("c", C.WHITE);
                                    this.out.write(C.WHITE);
                                    break;
                                case TT.Yellow:
                                    this.state.push("c", C.YELLOW);
                                    this.out.write(C.YELLOW);
                                    break;
                                case TT.Rgb:
                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.LeftParen)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const r: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Comma)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const g: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Comma)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const b: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.RightParen)
                                        throw new ParseError();

                                    const rgb = C.rgb(r, g, b);
                                    this.state.push("c", rgb);
                                    this.out.write(rgb);
                                    break;
                                default:
                                    throw new ParseError();
                            }
                            break;
                        default:
                            throw new ParseError();
                    }
                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;

                case TT.X:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case TT.Dot:
                            token = this.scanner.scanToken();
                            switch (token.kind) {
                                case TT.Black:
                                    this.state.push("x", X.BLACK);
                                    this.out.write(X.BLACK);
                                    break;
                                case TT.Blue:
                                    this.state.push("x", X.BLUE);
                                    this.out.write(X.BLUE);
                                    break;
                                case TT.Cyan:
                                    this.state.push("x", X.CYAN);
                                    this.out.write(X.CYAN);
                                    break;
                                case TT.Green:
                                    this.state.push("x", X.GREEN);
                                    this.out.write(X.GREEN);
                                    break;
                                case TT.Magenta:
                                    this.state.push("x", X.MAGENTA);
                                    this.out.write(X.MAGENTA);
                                    break;
                                case TT.Red:
                                    this.state.push("x", X.RED);
                                    this.out.write(X.RED);
                                    break;
                                case TT.White:
                                    this.state.push("x", X.WHITE);
                                    this.out.write(X.WHITE);
                                    break;
                                case TT.Yellow:
                                    this.state.push("x", X.YELLOW);
                                    this.out.write(X.YELLOW);
                                    break;
                                case TT.Rgb:
                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.LeftParen)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const r: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Comma)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const g: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Comma)
                                        throw new ParseError();

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.Number)
                                        throw new ParseError();
                                    const b: number = parseInt(token.content);

                                    token = this.scanner.scanToken();
                                    if (token.kind != TT.RightParen)
                                        throw new ParseError();

                                    const rgb = X.rgb(r, g, b);
                                    this.state.push("x", rgb);
                                    this.out.write(rgb);
                                    break;
                                default:
                                    throw new ParseError();
                            }
                            break;
                        default:
                            throw new ParseError();
                    }
                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.Slash:
                    token = this.scanner.scanToken();
                    switch (token.kind) {
                        case TT.B:
                            if (this.state.currentTag() != "b")
                                throw new ParseError();
                            this.state.pop();
                            this.out.write(RESET_B);
                            break;
                        case TT.I:
                            if (this.state.currentTag() != "i")
                                throw new ParseError();
                            this.state.pop();
                            this.out.write(RESET_I);
                            break;
                        case TT.S:
                            if (this.state.currentTag() != "s")
                                throw new ParseError();
                            this.state.pop();
                            this.out.write(RESET_S);
                            break;
                        case TT.U:
                            if (this.state.currentTag() != "u")
                                throw new ParseError();
                            this.state.pop();
                            this.out.write(RESET_U);
                            break;
                        case TT.C: {
                            if (this.state.currentTag() != "c")
                                throw new ParseError();
                            this.state.pop();
                            let saved = this.state.currentSave();
                            this.out.write(saved);
                            break;
                        }
                        case TT.X: {
                            if (this.state.currentTag() != "x")
                                throw new ParseError();
                            this.state.pop();
                            let saved = this.state.currentSave();
                            this.out.write(saved);
                            break;
                        }
                        case TT.Identifier:
                        case TT.Black:
                        case TT.Blue:
                        case TT.Cyan:
                        case TT.Green:
                        case TT.Magenta:
                        case TT.Red:
                        case TT.Rgb:
                        case TT.White:
                        case TT.Yellow: {
                            if (this.state.currentTag() != token.content)
                                throw new ParseError();
                            this.state.pop();
                            let saved = this.state.currentSave();
                            this.out.write(saved);
                            break;
                        }
                        default:
                            throw new ParseError();
                    }

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.Identifier:
                case TT.Black:
                case TT.Blue:
                case TT.Cyan:
                case TT.Green:
                case TT.Magenta:
                case TT.Red:
                case TT.Rgb:
                case TT.White:
                case TT.Yellow:
                    const val = this.variables[token.content];
                    if (typeof val != "undefined") {
                        this.state.push(token.content, val);
                        this.out.write(val);
                    } else {
                        throw new ParseError();
                    }

                    token = this.scanner.scanToken();
                    if (token.kind != TT.CloseTag) throw new ParseError();
                    break;
                case TT.Error:
                    throw new ParseError("Unexpected character. " + token.content)
                case TT.Eof:
                    this.out.write(RESET);
                    return;
                default:
                    continue;
            }
        }
    }
}
