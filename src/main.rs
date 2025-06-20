use arg::{parse_args, Cli};
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, IsTerminal, Read, Write};
use std::path::Path;
use std::process::exit;
use std::rc::Rc;
use ziyy::Error;
use ziyy_core::{Document, Indexer, Parser, Resolver, Result, Splitter};

mod arg;

fn parse(source: &str) -> Result<Rc<Document>> {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.to_string());
    let mut splitter = Splitter::new();
    let frags = splitter.split(source);

    let parser = Parser::new();
    let chunks = parser.parse(frags)?;

    let mut resolver = Resolver::new();
    Ok(resolver.resolve(chunks))
}

fn parse_to_out(source: &str, out: &mut impl Write, options: Options) {
    let mut f = || {
        let output = parse(source)?;
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
    let help = parse(&format!(
        include_str!("help.zi"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_BIN_NAME")
    ))
    .unwrap();

    if !out.is_terminal() {
        help.root().null_tags();
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
            long_switches: &["help", "version", "tree"],
        },
    )
    .unwrap();
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
        } else if arg.is_short_switch_and(|s| s == "c") {
            options.cli = true;
        } else if arg.is_short_switch_and(|s| s == "n") {
            options.no_newline = true;
        } else if arg.is_long_switch_and(|s| s == "tree") {
            options.tree = true;
        } else {
            arg.is_params_and(|s| params.push(s))
        }
    }

    if options.cli {
        parse_to_out(&params.join(" "), &mut out, options);
        if !options.no_newline {
            let _ = writeln!(out);
        }
    } else {
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

    if params.is_empty() {
        usage();
        exit(1);
    }

    let _ = stdout.write(&out);
}

#[derive(Default, Clone, Copy)]
struct Options {
    no_newline: bool,
    cli: bool,
    tree: bool,
}
