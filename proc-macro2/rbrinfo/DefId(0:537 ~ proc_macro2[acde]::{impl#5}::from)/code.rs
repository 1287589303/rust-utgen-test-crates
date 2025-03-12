fn from(token: TokenTree) -> Self {
        TokenStream::_new(imp::TokenStream::from(token))
    }