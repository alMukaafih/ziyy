use std::io::stdout;
use std::fs::File;

use ziyy::compile;

fn main() {
    let mut out_1 = stdout().lock();
    let mut out_2 = File::create("foo.txt").unwrap();
    compile(include_str!("help.z"), &mut out_1);
    compile(include_str!("help.z"), &mut out_2);
}