use proc_macro::{TokenStream, TokenTree};

/// Include source from file.
#[proc_macro]
pub fn source(item: TokenStream) -> TokenStream {
    let token = item.into_iter().next().unwrap();
    let s;
    match token {
        TokenTree::Literal(lit) => {
            s = lit.to_string();
        }
        _ => panic!(),
    }

    let s = s.strip_prefix('"').unwrap();
    let s = s.strip_suffix('"').unwrap();
    let mut s = s.to_owned();

    if !s.ends_with(".zi") {
        s = format!("{s}.zi");
    }
    format!("include_str!(\"{s}\")").parse().unwrap()
}
