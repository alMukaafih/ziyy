import { Token, TokenKind } from "./token";

function isAlpha(c: string): boolean {
    const m = c.match(/[a-zA-Z_]/);
    if (m) return true;
    else return false;
}

function isDigit(c: string): boolean {
    const m = c.match(/[0-9]/);
    if (m) return true;
    else return false;
}

export class Scanner {
    source: string;
    start: number = 0;
    current: number = 0;
    line: number = 1;
    textLine: number = 1;
    textMode: boolean = true;
    escape: number = 0;

    constructor(source: string) {
        this.source = source;
    }

    isAtEnd(): boolean {
        return this.current + 1 > this.source.length;
    }

    advance(): string {
        this.current++;
        return this.source[this.current - 1];
    }

    peek(): string {
        try {
            return this.source[this.current];
        } catch {
            return "\0";
        }
    }

    peekNext(): string {
        try {
            return this.source[this.current + 1];
        } catch {
            return "\0";
        }
    }

    makeToken(kind: TokenKind): Token {
        const s = this.source.slice(this.start, this.current);
        return new Token(kind, s, 0, this.line);
    }

    errorToken(code: number): Token {
        const s = this.source.slice(this.start, this.current);
        return new Token(TokenKind.Error, s, code, this.line);
    }

    textToken(): Token {
        const s = this.source.slice(this.start, this.current);
        const token = new Token(TokenKind.Text, s, 0, this.textLine);
        this.textLine = this.line;
        return token;
    }

    skipWhitespace() {
        for (;;) {
            if (this.textMode) {
                return;
            }
            const c = this.peek();
            switch (c) {
                case " ":
                case "\r":
                case "\t":
                    this.advance();
                    continue;
                case "\n":
                    this.line += 1;
                    this.textLine += 1;
                    this.advance();
                    continue;
                default:
                    return;
            }
        }
    }

    checkKeyword(
        start: number,
        length: number,
        rest: string,
        kind: TokenKind
    ): TokenKind {
        const s = this.source.slice(this.start + start, this.current);
        if (this.current - this.start === start + length && s === rest) {
            return kind;
        } else {
            return TokenKind.Identifier;
        }
    }

    identifierKind(): TokenKind {
        if (this.current - this.start === 1) {
            switch (this.source[this.start]) {
                case "b":
                    return TokenKind.B;
                case "c":
                    return TokenKind.C;
                case "i":
                    return TokenKind.I;
                case "s":
                    return TokenKind.S;
                case "u":
                    return TokenKind.U;
                case "x":
                    return TokenKind.X;
                default:
                    return TokenKind.Identifier;
            }
        } else {
            switch (this.source[this.start]) {
                case "b":
                    switch (this.source[this.start + 1]) {
                        case "l":
                            switch (this.source[this.start + 2]) {
                                case "a":
                                    return this.checkKeyword(
                                        3,
                                        2,
                                        "ck",
                                        TokenKind.Black
                                    );
                                case "u":
                                    return this.checkKeyword(
                                        3,
                                        1,
                                        "e",
                                        TokenKind.Blue
                                    );
                                default:
                                    return TokenKind.Identifier;
                            }
                    }
                case "c":
                    return this.checkKeyword(1, 3, "yan", TokenKind.Cyan);
                case "g":
                    return this.checkKeyword(1, 4, "reen", TokenKind.Green);
                case "m":
                    return this.checkKeyword(1, 6, "agenta", TokenKind.Magenta);
                case "r":
                    switch (this.source[this.start + 1]) {
                        case "e":
                            return this.checkKeyword(2, 1, "d", TokenKind.Red);
                        case "g":
                            return this.checkKeyword(2, 1, "b", TokenKind.Rgb);
                        default:
                            return TokenKind.Identifier;
                    }
                case "w":
                    return this.checkKeyword(1, 4, "hite", TokenKind.White);
                case "y":
                    return this.checkKeyword(1, 5, "ellow", TokenKind.Yellow);
                default:
                    return TokenKind.Identifier;
            }
        }
    }

    identifier(): Token {
        while (isAlpha(this.peek()) || isDigit(this.peek())) {
            this.advance();
        }

        const kind = this.identifierKind();
        return this.makeToken(kind);
    }

    number(): Token {
        while (isDigit(this.peek())) {
            this.advance();
        }

        return this.makeToken(TokenKind.Number);
    }

    scanToken(): Token {
        if (this.escape === 0) {
            this.skipWhitespace();
        }
        this.start = this.current;
        if (this.escape === 2 && this.peek() === "\\") {
            this.escape = 1;
            this.advance();
            return this.makeToken(TokenKind.BackSlash);
        }
        if (this.escape === 1) {
            this.escape = 0;
            this.advance();
            return this.textToken();
        }
        if (this.isAtEnd()) {
            return this.makeToken(TokenKind.Eof);
        }

        const c = this.advance();
        if (c === "<") {
            this.textMode = false;
            return this.makeToken(TokenKind.OpenTag);
        } else if (c === ">") {
            this.textMode = true;
            return this.makeToken(TokenKind.CloseTag);
        }

        if (this.textMode) {
            while (!this.isAtEnd()) {
                if (this.peek() === "\n") {
                    this.line += 1;
                }
                if (this.peek() === "\\") {
                    this.escape = 2;
                    return this.textToken();
                }
                if (this.peek() != "<") {
                    this.advance();
                } else {
                    break;
                }
            }
            return this.textToken();
        }

        if (isAlpha(c)) {
            return this.identifier();
        }

        if (isDigit(c)) {
            return this.number();
        }

        switch (c) {
            case "(":
                return this.makeToken(TokenKind.LeftParen);
            case ")":
                return this.makeToken(TokenKind.RightParen);
            case ",":
                return this.makeToken(TokenKind.Comma);
            case ".":
                return this.makeToken(TokenKind.Dot);
            case "/":
                return this.makeToken(TokenKind.Slash);
            default:
                return this.errorToken(1);
        }
    }
}
