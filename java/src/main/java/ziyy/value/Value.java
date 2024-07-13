package ziyy.value;

public class Value {
    static final String esc = Character.toString('\033');

    /** Bold Value. */
    public static final String B = esc + "[1m";
    /** Italics Value. */
    public static final String I = esc + "[3m";
    /** Underline Value. */
    public static final String U = esc + "[4m";
    /** Strike through Value. */
    public static final String S = esc + "[9m";

    public static final String RESET = esc + "[0m";
    public static final String RESET_B = esc + "[22m";
    public static final String RESET_I = esc + "[23m";
    public static final String RESET_U = esc + "[24m";
    public static final String RESET_S = esc + "[29m";
    public static final String RESET_C = esc + "[39m";
    public static final String RESET_X = esc + "[49m";
}
