pub fn push_lifetime_spanned(tokens: &mut TokenStream, span: Span, lifetime: &str) {
    struct Lifetime<'a> {
        name: &'a str,
        span: Span,
        state: u8,
    }

    impl<'a> Iterator for Lifetime<'a> {
        type Item = TokenTree;

        fn next(&mut self) -> Option<Self::Item> {
            match self.state {
                0 => {
                    self.state = 1;
                    let mut apostrophe = Punct::new('\'', Spacing::Joint);
                    apostrophe.set_span(self.span);
                    Some(TokenTree::Punct(apostrophe))
                }
                1 => {
                    self.state = 2;
                    Some(TokenTree::Ident(Ident::new(self.name, self.span)))
                }
                _ => None,
            }
        }
    }

    tokens.extend(Lifetime {
        name: &lifetime[1..],
        span,
        state: 0,
    });
}