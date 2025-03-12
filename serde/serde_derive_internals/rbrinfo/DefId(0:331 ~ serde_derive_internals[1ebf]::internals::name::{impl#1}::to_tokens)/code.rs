fn to_tokens(&self, tokens: &mut TokenStream) {
        LitStr::new(&self.value, self.span).to_tokens(tokens);
    }