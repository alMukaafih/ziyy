use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::parse::{End, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, LitStr, Token, parse_macro_input};

struct StyleFmt {
    source: LitStr,
    exprs: Option<Punctuated<Expr, Token![,]>>,
}

impl Parse for StyleFmt {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut source: LitStr = input.parse()?;
        let lookahead = input.lookahead1();
        let exprs;
        if lookahead.peek(Token![,]) {
            let comma = input.parse::<Token![,]>()?;
            source.set_span(comma.span);
            exprs = Some(input.parse_terminated(Expr::parse, Token![,])?);
        } else if lookahead.peek(End) {
            exprs = None
        } else {
            return Err(lookahead.error());
        }

        Ok(Self { source, exprs })
    }
}

/// Styles formatted text
#[proc_macro]
pub fn style_fmt(tokens: TokenStream) -> TokenStream {
    let StyleFmt {
        source,
        exprs: idents,
    } = parse_macro_input!(tokens as StyleFmt);

    let span = source.span();
    let parsed = match ziyy_core::try_style(source.value()) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };

    let expanded = quote_spanned! {
        span => format!(#parsed, #idents)
    };

    TokenStream::from(expanded)
}

/// Styles text
#[proc_macro]
pub fn style(tokens: TokenStream) -> TokenStream {
    let source = parse_macro_input!(tokens as LitStr);
    let span = source.span();
    let parsed = match ziyy_core::try_style(source.value()) {
        Ok(s) => s,
        Err(e) => panic!("{e}"),
    };

    let expanded = quote_spanned! {
        span => #parsed
    };

    TokenStream::from(expanded)
}
