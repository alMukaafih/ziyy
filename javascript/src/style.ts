import { WriteStream } from "tty";
import { Compiler } from "./compiler";
import { C } from "./value";

class String extends WriteStream {
    s: string;

    constructor() {
        super(1)
        this.s = ""
    }

    write(buffer: string): boolean {
        this.s += buffer
        return true
    }
}

export function compile(source: string, out: NodeJS.WriteStream) {
    const vars = {
        "green": C.rgb(0, 150, 75),
        "cyan": C.rgb(0, 150, 150)
    }
    const compiler = new Compiler(source, out, vars)
    compiler.compile()
}

export function style(text: string): string {
    const vars = {}
    const out = new String
    const compiler = new Compiler(text, out, vars)
    compiler.compile()
    return out.s
}


export function template(save: string) {
    return function(text: string): string {
        return style(`${save}${text}`)
    }
}

if (require.main === module) {
    console.log(style("[b]Hello[/b] World!"))
}
