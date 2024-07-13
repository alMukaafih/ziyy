package ziyy.scanner;

public class Scanner {
    String source;
    int start = 0;
    int current = 0;
    int line = 1;
    int textLine = 1;
    boolean textMode = true;
    int escape = 0;

    boolean isAlpha(char c) {
        return (c >= 'a' && c <= 'z') ||
                (c >= 'A' && c <= 'Z') ||
                c == '_';
    }

    boolean isDigit(char c) {
        return c >= '0' && c <= '9';
    }

    public Scanner(String source) {
        this.source = source;
    }

    boolean isAtEnd() {
        return current + 1 > source.length();
    }

    char advance() {
        current++;
        return source.charAt(current - 1);
    }

    char peek() {
        try {
            return source.charAt(current);
        } catch (Exception e) {
            return '\0';
        }
    }

    char peekNext() {
        try {
            return source.charAt(current + 1);
        } catch (Exception e) {
            return '\0';
        }
    }

    Token makeToken(TokenKind kind) {
        String s = source.substring(start, current);
        return new Token(kind, s, 0, line);
    }

    Token errorToken(int code) {
        String s = source.substring(start, current);
        return new Token(TokenKind.Error, s, code, line);
    }

    Token textToken() {
        String s = source.substring(start, current);
        Token token = new Token(TokenKind.Text, s, 0, line);
        textLine = line;
        return token;
    }

    void skipWhitespace() {
        for (;;) {
            if (textMode) {
                return;
            }
            char c = peek();
            switch (c) {
                case ' ':
                case '\r':
                case '\t':
                    advance();
                    continue;
                case '\n':
                    line += 1;
                    textLine += 1;
                    advance();
                    continue;
                default:
                    return;
            }
        }
    }

    TokenKind checkKeyword(
            int start,
            int length,
            String rest,
            TokenKind kind) {
        String s = source.substring(this.start + start, current);
        if (this.current - this.start == start + length && s.equals(rest)) {
            return kind;
        } else {
            return TokenKind.Identifier;
        }
    }

    TokenKind identifierKind() {
        if (this.current - this.start == 1) {
            switch (this.source.charAt(start)) {
                case 'b':
                    return TokenKind.B;
                case 'c':
                    return TokenKind.C;
                case 'i':
                    return TokenKind.I;
                case 's':
                    return TokenKind.S;
                case 'u':
                    return TokenKind.U;
                case 'x':
                    return TokenKind.X;
                default:
                    return TokenKind.Identifier;
            }
        } else {
            switch (source.charAt(start)) {
                case 'b':
                    switch (source.charAt(start + 1)) {
                        case 'l':
                            switch (source.charAt(start + 2)) {
                                case 'a':
                                    return checkKeyword(
                                            3,
                                            2,
                                            "ck",
                                            TokenKind.Black);
                                case 'u':
                                    return checkKeyword(
                                            3,
                                            1,
                                            "e",
                                            TokenKind.Blue);
                                default:
                                    return TokenKind.Identifier;
                            }
                    }
                case 'c':
                    return checkKeyword(1, 3, "yan", TokenKind.Cyan);
                case 'g':
                    return checkKeyword(1, 4, "reen", TokenKind.Green);
                case 'm':
                    return checkKeyword(1, 6, "agenta", TokenKind.Magenta);
                case 'r':
                    switch (source.charAt(start + 1)) {
                        case 'e':
                            return checkKeyword(2, 1, "d", TokenKind.Red);
                        case 'g':
                            return checkKeyword(2, 1, "b", TokenKind.Rgb);
                        default:
                            return TokenKind.Identifier;
                    }
                case 'w':
                    return checkKeyword(1, 4, "hite", TokenKind.White);
                case 'y':
                    return checkKeyword(1, 5, "ellow", TokenKind.Yellow);
                default:
                    return TokenKind.Identifier;
            }
        }
    }

    Token identifier() {
        while (isAlpha(peek()) || isDigit(peek())) {
            advance();
        }

        TokenKind kind = identifierKind();
        return makeToken(kind);
    }

    Token number() {
        while (isDigit(peek())) {
            advance();
        }

        return makeToken(TokenKind.Number);
    }

    public Token scanToken() {
        if (escape == 0) {
            skipWhitespace();
        }
        start = current;
        if (escape == 2 && peek() == '\\') {
            escape = 1;
            advance();
            return makeToken(TokenKind.BackSlash);
        }
        if (escape == 1) {
            escape = 0;
            advance();
            return textToken();
        }
        if (isAtEnd()) {
            return makeToken(TokenKind.Eof);
        }

        char c = advance();
        if (c == '<') {
            textMode = false;
            return makeToken(TokenKind.OpenTag);
        } else if (c == '>') {
            textMode = true;
            return makeToken(TokenKind.CloseTag);
        }

        if (textMode) {
            while (!isAtEnd()) {
                if (peek() == '\n') {
                    line++;
                }
                if (peek() == '\\') {
                    escape = 2;
                    return textToken();
                }
                if (peek() != '<') {
                    advance();
                } else {
                    break;
                }
            }
            return textToken();
        }

        if (isAlpha(c)) {
            return identifier();
        }

        if (isDigit(c)) {
            return number();
        }

        switch (c) {
            case '(':
                return makeToken(TokenKind.LeftParen);
            case ')':
                return makeToken(TokenKind.RightParen);
            case ',':
                return makeToken(TokenKind.Comma);
            case '.':
                return makeToken(TokenKind.Dot);
            case '/':
                return makeToken(TokenKind.Slash);
            default:
                return errorToken(1);
        }
    }
}
