use proc_macro2::{TokenStream, TokenTree, Span};
use syn::{LitStr};
use syn::parse::{Parse, Parser, ParseStream};

fn parse_with<F, T>(lit_str: &syn::LitStr, parser: F) -> syn::Result<T>
where
    F: Parser<Output=T>
{
    use proc_macro2::Group;
    fn spanned_tokens(s: &LitStr) -> Result<TokenStream, syn::Error> {
        let stream = syn::parse_str(&s.value())?;
        Ok(respan_token_stream(stream, s.span()))
    }

    // Token stream with every span replaced by the given one.
    fn respan_token_stream(stream: TokenStream, span: Span) -> TokenStream {
        stream
            .into_iter()
            .map(|token| respan_token_tree(token, span))
            .collect()
    }

    // Token tree with every span replaced by the given one.
    fn respan_token_tree(mut token: proc_macro2::TokenTree, span: Span) -> proc_macro2::TokenTree {
        match token {
            proc_macro2::TokenTree::Group(ref mut g) => {
                let stream = respan_token_stream(g.stream().clone(), span);
                *g = Group::new(g.delimiter(), stream);
                g.set_span(span);
            }
            ref mut other => other.set_span(span),
        }
        token
    }
    spanned_tokens(lit_str).and_then(|stream| parser.parse2(stream))
}