package ziyy;

import java.io.FileNotFoundException;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.util.HashMap;
import java.util.Map;

import ziyy.compiler.Compiler;
import ziyy.compiler.ParseException;
import ziyy.value.C;


class Str extends PrintStream {
    String s;

    public Str(String arg0) throws FileNotFoundException {
        super(arg0);
        s = arg0;
    }

    @Override
    public void print(String arg0) {
        s += arg0;
    }
}

public class Ziyy {
    static void compile(String source,
            PrintStream out) throws ParseException {
        Map<String, String> vars = new HashMap<String, String>();
        vars.put("green", C.rgb(0, 150, 75));
        vars.put("cyan", C.rgb(0, 150, 150));

        Compiler compiler = new Compiler(source, out, vars);
        compiler.compile();
    }

    String style(String text) throws ParseException {
        Str out = null;
        try {
            out = new Str("");
        } catch (FileNotFoundException e) {
            e.printStackTrace();
        }
        Compiler compiler = new Compiler(text, out, null);
        compiler.compile();
        return out.s;
    }

    static void usage() throws ParseException {
        compile("""
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

    public static void main(String[] args) throws ParseException, IOException {
        PrintStream out = System.out;
        if (args.length < 1) {
            usage();
            System.exit(0);
        }
        String first = args[0];
        if (first.equals("-n") || first.equals("--no-newline"))
            compile(args[1], out);
        else if (first.equals("-f") || first.equals("--file")) {
            if (args.length == 1) {
                usage();
                System.exit(1);
            }
            FileSystem fs = FileSystems.getDefault();
            if (!Files.isRegularFile(fs.getPath(args[1]))) {
                usage();
                System.exit(1);
            }
            String file = Files.readString(fs.getPath(args[1]));
            compile(file, out);
        }
        else if (first.equals("-V") || first.equals("--version")) {
            System.out.println("ziyy 2.0.0-beta.0");
        }
        else if (first.equals("-h") || first.equals("--help")) {
            usage();
            System.exit(0);
        }
        else {
            compile(first, out);
            out.print("\n");
        }
    }
}
