package ziyy.compiler;

import java.io.PrintStream;
import java.util.Map;

public class Compiler {
    private Parser parser;

    public Compiler(
            String source,
            PrintStream out,
            Map<String, String> variables) {
        parser = new Parser(source, out, variables);
    }

    public void compile() throws ParseException {
        parser.parseToOut();
    }
}
