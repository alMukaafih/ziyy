use arg::parse_args;
use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::Parser;

mod arg;

pub fn parse(source: &str, out: &mut impl Write) -> ziyy::Result<()> {
    let mut parser = Parser::new(source, None);
    let result = parser.parse_to_bytes()?;
    let _ = out.write(&result);
    Ok(())
}

fn usage() {
    let mut out = stdout().lock();
    parse(include_str!("../../../../help.zi"), &mut out).unwrap();
}

fn main() {
    let mut args0 = env::args();
    let mut out: Vec<u8> = vec![];
    if args0.len() - 1 < 1 {
        usage();
        exit(0);
    }

    args0.next();
    let args = parse_args(args0);
    let mut opt = Opt::default();
    let mut param = None;
    let mut first_param = true;

    //println!("{args:?}");
    for arg in args {
        match arg.r#type {
            arg::ArgType::Param => {
                if first_param {
                    param = Some(arg.key);
                    first_param = false;
                }
            }

            arg::ArgType::Flag => match arg.key.as_str() {
                "c" => {
                    opt.cli = true;
                    param = arg.value;
                    first_param = false;
                }

                "n" => {
                    opt.no_newline = true;
                }
                _ => {}
            },
        }
    }

    if param.is_none() {
        usage();
        exit(1);
    }

    if opt.cli {
        if let Err(err) = parse(&param.unwrap(), &mut out) {
            println!("{err:?}");
            exit(1)
        }
        if opt.no_newline {
            let _ = stdout().write(&out);
        } else {
            let _ = writeln!(out);
            let _ = stdout().write(&out);
        }
    } else {
        let path = param.unwrap();
        if !Path::new(&path).is_file() {
            usage();
            exit(1);
        }
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let mut file = String::new();
        let _ = reader.read_to_string(&mut file);
        if file.starts_with("#!") {
            let mut lines = file.split_inclusive('\n');
            lines.next();
            file = lines.collect::<Vec<_>>().join("\n");
        }
        if let Err(err) = parse(&file, &mut out) {
            println!("{err:?}");
            exit(1)
        }
        let _ = stdout().write(&out);
    }
}

#[derive(Default)]
struct Opt {
    no_newline: bool,
    cli: bool,
}
