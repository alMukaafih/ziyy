use std::{ffi::OsStr, path::PathBuf};

use proc_macro::{Literal, TokenStream, TokenTree};

#[proc_macro]
pub fn import(item: TokenStream) -> TokenStream {
    let token = item.into_iter().next().unwrap();
    let s;
    match token {
        TokenTree::Literal(lit) => {
            s = lit.to_string();
        },
        _ => panic!(),
    }

    let s = s.strip_prefix('"').unwrap();
    let s = s.strip_suffix('"').unwrap();

    let paths = s.split('/');
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for path in paths {
        p.push(path);
    }
    p.set_extension("z");
    let os_s: &OsStr = p.as_ref();
    let s = os_s.to_str().unwrap();
    let token = TokenTree::Literal(Literal::string(s));

    token.into()
}

// #[test]
// fn test_resolve() {
//     let i = resolve!("");

// }