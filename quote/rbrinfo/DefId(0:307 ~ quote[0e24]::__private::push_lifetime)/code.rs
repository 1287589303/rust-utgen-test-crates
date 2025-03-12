pub fn push_lifetime(tokens: &mut TokenStream, lifetime: &str) {
    struct Lifetime<'a> {
        name: &'a str,
        state: u8,
    }

    impl<'a> Iterator for Lifetime<'a> {
        type Item = TokenTree;

        fn next(&mut self) -> Option<Self::Item> {
            match self.state {
                0 => {
                    self.state = 1;
                    Some(TokenTree::Punct(Punct::new('\'', Spacing::Joint)))
                }
                1 => {
                    self.state = 2;
                    Some(TokenTree::Ident(Ident::new(self.name, Span::call_site())))
                }
                _ => None,
            }
        }
    }

    tokens.extend(Lifetime {
        name: &lifetime[1..],
        state: 0,
    });
}