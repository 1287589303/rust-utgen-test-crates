fn from_str(src: &str) -> Result<TokenStream, LexError> {
        match imp::TokenStream::from_str_checked(src) {
            Ok(tokens) => Ok(TokenStream::_new(tokens)),
            Err(lex) => Err(LexError {
                inner: lex,
                _marker: MARKER,
            }),
        }
    }