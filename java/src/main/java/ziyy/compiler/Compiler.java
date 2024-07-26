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

    public void compileSource(String source) throws ParseException {
        parser.scanner.source = source;
        compile();
    }
}
