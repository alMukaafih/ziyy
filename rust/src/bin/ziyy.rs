use ziyy::style;
use std::env;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn usage() -> String {
    style("Convenient Terminal Output Styler.

[c:green][b]Usage: [c:cyan]ziyy[/b] [c:cyan][OPTION] [TEXT]

[b][c:green]Options:[/0]
  [c:cyan][b]-V[/0], [c:cyan][b]--version[/0]
          Print version info and exit
  [c:cyan][b]-f[/0], [c:cyan][b]--file[/b] <FILENAME>[/c]
          Read input from file.
  [c:cyan][b]-n[/0], [c:cyan][b]--no-newline[/0]
          Do not print newline after text.
  [c:cyan][b]-h[/0], [c:cyan][b]--help[/0]
          Print help
")
}

fn main() {
    let mut args = env::args();
    if args.len() - 1 < 1 {
        print!("{}", usage());
        exit(1);
    }
    let first = args.nth(1);
    if first == Some("-n".to_string()) || first == Some("--no-newline".to_string()) {
        print!("{}", style(args.nth(0).unwrap().as_str()))
    } else if first == Some("-f".to_string()) || first == Some("--file".to_string()) {
        let f = File::open(args.nth(0).unwrap()).unwrap();
        let mut reader = BufReader::new(f);
        let mut file = String::new();
        let _ = reader.read_to_string(&mut file);
        print!("{}", style(file.as_str()))
    } else if first == Some("-V".to_string()) || first == Some("--version".to_string()) {
        println!("ziyy 0.1.1")
    } else if first == Some("-h".to_string()) || first == Some("--help".to_string()) {
        print!("{}", usage());
        exit(0);
    } else {
        println!("{}", style(first.unwrap().as_str()))
    }
}