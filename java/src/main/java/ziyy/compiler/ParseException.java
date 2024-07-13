package ziyy.compiler;

public class ParseException extends Exception {
    ParseException() {
    }

    ParseException(String desc) {
        super(desc);
    }
}