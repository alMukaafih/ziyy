import { Parser } from "./parser";

export class Compiler {
    private parser: Parser;

    constructor(
        source: string,
        out: NodeJS.WriteStream, variables: Record<string, string>
    ) {
        this.parser = new Parser(source, out, variables)
    }

    compile() {
        this.parser.parseToOut()
    }
}