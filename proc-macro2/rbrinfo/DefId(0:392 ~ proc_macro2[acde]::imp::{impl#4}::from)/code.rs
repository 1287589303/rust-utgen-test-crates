fn from(inner: TokenStream) -> Self {
        match inner {
            TokenStream::Compiler(inner) => inner.into_token_stream(),
            TokenStream::Fallback(inner) => {
                proc_macro::TokenStream::from_str_unchecked(&inner.to_string())
            }
        }
    }