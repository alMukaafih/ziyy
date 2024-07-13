package ziyy.value;

public class C {
    static final String esc = Character.toString('\033');
    /** Black Color. */
    public static final String BLACK   = esc + "[30m";
    /** Red Color. */
    public static final String RED     = esc + "[31m";
    /** Green Color. */
    public static final String GREEN   = esc + "[32m";
    /** Yellow Color. */
    public static final String YELLOW  = esc + "[33m";
    /** Blue Color. */
    public static final String BLUE    = esc + "[34m";
    /** Magenta Color. */
    public static final String MAGENTA = esc + "[35m";
    /** Cyan Color. */
    public static final String CYAN    = esc + "[36m";
    /** White Color. */
    public static final String WHITE   = esc + "[37m";

    /** RGB Color. */
    public static String rgb(int r, int g, int b) {
        return esc + "[38;2;" + r + ";" + g + ";" + b + "m";
    }
}
