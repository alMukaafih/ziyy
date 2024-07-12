import { readFileSync, statSync } from "fs";
import { compile } from "./style";

const out = process.stdout

function usage() {
    compile(`Convenient Terminal Output Styler.

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
`, out)
}

const args = process.argv.slice(2)
if (args.length < 1) {
    usage()
    process.exit(0)
}
const first = args[0]
if (first == "-n" || first == "--no-newline")
    compile(args[1], out)
else if (first == "-f" || first == "--file") {
    if (args.length == 1)
        process.exit(1)
    let file = statSync(args[1])
    if (!file.isFile())
        process.exit()
    compile(readFileSync(args[1], "utf8"), out)
}
else if (first == "-V" || first == "--version") {
    console.log("ziyy 1.0.6")
}
else if (first == "-h" || first == "--help") {
    usage()
    process.exit()
}
else {
    compile(first, out)
}