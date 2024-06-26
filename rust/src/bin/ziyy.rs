use std::env;
use std::fs::File;
use std::io::{BufReader, Read, stdout};
use std::process::exit;
use ziyy::compile;

fn usage() {
    let mut out = stdout().lock();
    compile(include_str!("help.z"), &mut out);
}

fn main() {
    let mut args = env::args();
    let mut out = stdout().lock();
    if args.len() - 1 < 1 {
        usage();
        exit(1);
    }
    let first = args.nth(1);
    if first == Some("-n".to_string()) || first == Some("--no-newline".to_string()) {
        compile(args.next().unwrap().as_str(), &mut out)
    } else if first == Some("-f".to_string()) || first == Some("--file".to_string()) {
        let f = File::open(args.next().unwrap()).unwrap();
        let mut reader = BufReader::new(f);
        let mut file = String::new();
        let _ = reader.read_to_string(&mut file);
        compile(file.as_str(), &mut out)
    } else if first == Some("-V".to_string()) || first == Some("--version".to_string()) {
        println!("ziyy 1.0.6")
    } else if first == Some("-h".to_string()) || first == Some("--help".to_string()) {
        usage();
        exit(0);
    } else {
        compile(first.unwrap().as_str(), &mut out)
    }
}
