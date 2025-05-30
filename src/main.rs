use arg::{parse_args, Arg::*, Cli};
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::style_fmt;
use ziyy_core::{Indexer, Parser, Resolver, Splitter};

mod arg;

pub fn parse(source: &str, out: &mut impl Write) -> ziyy::Result<()> {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.to_string());
    let mut splitter = Splitter::new();
    let frags = splitter.split(source);

    let parser = Parser::new();
    let chunks = parser.parse(frags)?;

    let mut resolver = Resolver::new();
    let output = resolver.resolve(chunks);

    let mut buf = String::new();
    output.root().to_string(&mut buf);

    let _ = out.write(buf.as_bytes());
    Ok(())
}

fn usage() {
    let mut out = stdout();
    let help = style_fmt!(
        r#"<ziyy>
            <let name="bold:green" c="green" b />
            <let name="cyan" c="cyan" />

            <p>{}</p>
            <br />
            <p>
                <span src="bold:green">Usage:</span> <cyan><b>{}</b> <i>[OPTION]</i> \<FILE\></cyan>
            </p>
            <br />

            <p src="bold:green">Options:</p>
            <p tab="2" src="cyan" b>-V<e>,</e> --version</p>
            <p tab="10">Print version info and exit</p>
            <p tab="2" src="cyan" b>-h<e>,</e> --help</p>
            <p tab="10">Print help</p>
            <br />
        </ziyy>"#,
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_BIN_NAME")
    );

    let _ = out.write(help.as_bytes());
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
            long_switches: &["help", "version"],
        },
    )
    .unwrap();
    let mut opt = Opt::default();
    let mut params = vec![];
    println!("{args:?}");
    for arg in args {
        match arg {
            LongSwitch(switch) if switch == "help" => {
                usage();
                exit(0);
            }
            ShortSwitch(switch) if switch == "h" => {
                usage();
                exit(0);
            }

            LongSwitch(switch) if switch == "version" => {
                println!(env!("CARGO_PKG_VERSION")); // TODO: use
                exit(0);
            }
            ShortSwitch(switch) if switch == "V" => {
                println!(env!("CARGO_PKG_VERSION")); // TODO: use
                exit(0);
            }

            ShortSwitch(switch) if switch == "c" => {
                opt.cli = true;
            }

            ShortSwitch(switch) if switch == "n" => {
                opt.no_newline = true;
            }

            Param(param) => {
                params.push(param);
            }
            _ => {}
        }
    }

    if opt.cli {
        if let Err(err) = parse(&params.join(" "), &mut out) {
            println!("{err}");
            exit(1)
        }
        if !opt.no_newline {
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
            if let Err(err) = parse(&file, &mut out) {
                println!("{err}");
                exit(1)
            }
        }
    }

    if params.is_empty() {
        usage();
        exit(1);
    }

    let _ = stdout.write(&out);
}

#[derive(Default)]
struct Opt {
    no_newline: bool,
    cli: bool,
}
