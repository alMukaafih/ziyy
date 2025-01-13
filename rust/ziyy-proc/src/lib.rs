use core::iter::FromIterator;
use proc_macro::{Literal, TokenStream, TokenTree};

/// Styles text
#[proc_macro]
pub fn style(item: TokenStream) -> TokenStream {
    let mut tokens: Vec<_> = item.into_iter().collect();

    if tokens.len() > 0 {
        let token = tokens.get_mut(0).unwrap();

        match token {
            TokenTree::Literal(literal) => {
                let s: String = literal.to_string();
                let strings: Vec<_> = s.split('"').collect();
                let end = strings.len() - 1;
                let s = strings[1..end].join("\"");
                let parsed = ziyy_core::style(s);

                let literal: Literal = format!("r#\"{parsed}\"#").parse().unwrap();
                *token = TokenTree::Literal(literal)
            }
            _ => {}
        }
    }

    let token_stream = TokenStream::from_iter(tokens).to_string();
    format!("format!({token_stream})").parse().unwrap()
}
