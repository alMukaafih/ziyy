use std::io::stdout;

use ziyy::compile;

fn main() {
    let mut out = stdout().lock();
    compile(include_str!("help.z"), &mut out);
}