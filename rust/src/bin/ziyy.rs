use std::env;
use std::fs::File;
use std::io::{stdout, BufReader, Read, Write};
use std::path::Path;
use std::process::exit;
use ziyy::Compiler;

pub fn compile(source: &str, out: &mut impl Write) -> ziyy::Result<()> {
    let mut compiler = Compiler::new(source, out, None);
    compiler.compile()?;
    Ok(())
}

fn usage() {
    let mut out = stdout().lock();
    compile(include_str!("../../../help.zi"), &mut out).unwrap();
}

fn main() {
    let mut args = env::args();
    let mut out: Vec<u8> = vec![];
    if args.len() - 1 < 1 {
        usage();
        exit(0);
    }
    let first = args.nth(1);
    if first == Some("-n".to_string()) || first == Some("--no-newline".to_string()) {
        if compile(args.next().unwrap().as_str(), &mut out).is_err() {
            exit(1);
        }
    } else if first == Some("-f".to_string()) || first == Some("--file".to_string()) {
        if args.len() == 0 {
            usage();
            exit(1);
        }
        let file = args.next().unwrap();
        if !Path::new(&file).is_file() {
            usage();
            exit(1);
        }
        let f = File::open(file).unwrap();
        let mut reader = BufReader::new(f);
        let mut file = String::new();
        let _ = reader.read_to_string(&mut file);
        if compile(file.as_str(), &mut out).is_err() {
            exit(1)
        }
    } else if first == Some("-V".to_string()) || first == Some("--version".to_string()) {
        println!("ziyy 2.0.0-beta.2")
    } else if first == Some("-h".to_string()) || first == Some("--help".to_string()) {
        usage();
        exit(0);
    } else {
        if compile(first.unwrap().as_str(), &mut out).is_err() {
            exit(1)
        }
        let _ = writeln!(out);
        let _ = stdout().write(&out);
    }
}
