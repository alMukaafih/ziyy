use arg::{Cli, parse_args};
use std::env;
use std::fs::File;
use std::io::{BufReader, IsTerminal, Read, Write, stdin, stdout};
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use ziyy::Error;
use ziyy_core::{
    Document, Fragment, FragmentType, Indexer, Parser, Resolver, Result, Span, Splitter, WordParser,
};

mod arg;

fn parse_escapes_only(source: &str) -> Result<Rc<Document>> {
    let parser = WordParser::new();
    let span = Span::calculate(source);
    let chunks = parser.parse(Fragment::new(FragmentType::Word, source.to_string(), span))?;
    // println!("{chunks:?}");

    let mut resolver = Resolver::new(true);
    resolver.resolve(chunks)
}

fn parse(source: &str) -> Result<Rc<Document>> {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.to_string());
    let mut splitter = Splitter::new();
    let frags = splitter.split(source);

    let parser = Parser::default();
    let chunks = parser.parse(frags)?;

    let mut resolver = Resolver::new(false);
    resolver.resolve(chunks)
}

fn parse_to_out(source: &str, out: &mut impl Write, options: Options) {
    let mut f = || {
        let output = match options.escape_only {
            true => parse_escapes_only(source),
            false => parse(source),
        }?;

        if options.strip {
            output.root().strip_styles();
        }

        let mut buf = String::new();
        if options.tree {
            buf = output.to_string();
        } else {
            output.root().to_string(&mut buf);
        }

        let _ = out.write(buf.as_bytes());
        Ok::<(), Error>(())
    };
    if let Err(err) = f() {
        println!("{err}");
        exit(1)
    }
}

fn usage() {
    let mut out = stdout();
    let help = parse(&format!(include_str!("help.zi"), env!("CARGO_BIN_NAME"))).unwrap();

    if !out.is_terminal() {
        help.root().strip_styles();
    }

    let mut buf = String::new();
    help.root().to_string(&mut buf);
    let _ = out.write(buf.as_bytes());
    let _ = out.flush();
}

fn main() {
    let mut args0 = env::args();
    let mut out: Vec<u8> = vec![];
    let mut stdout = stdout().lock();
    if args0.len() - 1 < 1 {
        usage();
        exit(0);
    }

    args0.next();
    let args = parse_args(
        args0,
        Cli {
            short_flags: &[],
            long_flags: &["mode"],
            short_switches: &["h", "V", "c", "e", "n"],
            long_switches: &[
                "ansi",
                "cli",
                "help",
                "no-newline",
                "strip",
                "version",
                "tree",
            ],
        },
    )
    .unwrap(); // TODO: echo out error

    let mut options = Options::default();
    let mut params = vec![];
    //println!("{args:?}");
    for arg in args {
        if arg.is_long_switch_and(|s| s == "help") | arg.is_short_switch_and(|s| s == "h") {
            usage();
            exit(0);
        } else if arg.is_long_switch_and(|s| s == "version") | arg.is_short_switch_and(|s| s == "V")
        {
            println!(env!("CARGO_PKG_VERSION"));
            exit(0);
        } else if arg.is_short_switch_and(|s| s == "c") | arg.is_long_switch_and(|s| s == "cli") {
            options.cli = true;
        } else if arg.is_short_switch_and(|s| s == "e") | arg.is_long_switch_and(|s| s == "ansi") {
            options.escape_only = true;
        } else if arg.is_short_switch_and(|s| s == "n")
            | arg.is_long_switch_and(|s| s == "no-newline")
        {
            options.no_newline = true;
        } else if arg.is_long_switch_and(|s| s == "strip") {
            options.strip = true;
        } else if arg.is_long_switch_and(|s| s == "tree") {
            options.tree = true;
        } else {
            arg.is_params_and(|s| params.push(s))
        }
    }

    if options.cli {
        if params.is_empty() {
            let mut buf = String::new();
            let _ = stdin().read_to_string(&mut buf);
            parse_to_out(&buf, &mut out, options);
        } else {
            parse_to_out(&params.join(" "), &mut out, options);
        }
        if !options.no_newline {
            let _ = writeln!(out);
        }
    } else {
        if params.is_empty() {
            usage();
            exit(1);
        }
        for param in &params {
            if !Path::new(&param).is_file() {
                usage();
                exit(1);
            }
            let f = File::open(param).unwrap();
            let mut reader = BufReader::new(f);
            let mut file = String::new();
            let _ = reader.read_to_string(&mut file);
            if file.starts_with("#!") {
                let mut lines = file.split_inclusive('\n');
                lines.next();
                file = lines.collect::<Vec<_>>().join("\n");
            }
            parse_to_out(&file, &mut out, options)
        }
    }

    let _ = stdout.write(&out);
}

#[derive(Default, Clone, Copy)]
struct Options {
    cli: bool,
    escape_only: bool,
    no_newline: bool,
    strip: bool,
    tree: bool,
}
