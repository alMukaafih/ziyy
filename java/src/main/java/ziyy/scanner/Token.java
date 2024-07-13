package ziyy.scanner;

public class Token {
    public TokenKind kind;
    public String content;
    public int errCode;
    public int line;

    Token(
            TokenKind kind,
            String content,
            int errCode,
            int line) {
        this.kind = kind;
        this.content = content;
        this.errCode = errCode;
        this.line = line;
    }
}
