fn into_compiler_token(token: TokenTree) -> proc_macro::TokenTree {
    match token {
        TokenTree::Group(tt) => proc_macro::TokenTree::Group(tt.inner.unwrap_nightly()),
        TokenTree::Punct(tt) => {
            let spacing = match tt.spacing() {
                Spacing::Joint => proc_macro::Spacing::Joint,
                Spacing::Alone => proc_macro::Spacing::Alone,
            };
            let mut punct = proc_macro::Punct::new(tt.as_char(), spacing);
            punct.set_span(tt.span().inner.unwrap_nightly());
            proc_macro::TokenTree::Punct(punct)
        }
        TokenTree::Ident(tt) => proc_macro::TokenTree::Ident(tt.inner.unwrap_nightly()),
        TokenTree::Literal(tt) => proc_macro::TokenTree::Literal(tt.inner.unwrap_nightly()),
    }
}