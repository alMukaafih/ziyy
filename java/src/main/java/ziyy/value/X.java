package ziyy.value;

public class X {
    static final String esc = Character.toString('\033');
    /** Black Color. */
    public static final String BLACK   = esc + "[40m";
    /** Red Color. */
    public static final String RED     = esc + "[41m";
    /** Green Color. */
    public static final String GREEN   = esc + "[42m";
    /** Yellow Color. */
    public static final String YELLOW  = esc + "[43m";
    /** Blue Color. */
    public static final String BLUE    = esc + "[44m";
    /** Magenta Color. */
    public static final String MAGENTA = esc + "[45m";
    /** Cyan Color. */
    public static final String CYAN    = esc + "[46m";
    /** White Color. */
    public static final String WHITE   = esc + "[47m";

    /** RGB Color. */
    public static String rgb(int r, int g, int b) {
        return esc + "[48;2;" + r + ";" + g + ";" + b + "m";
    }
}
