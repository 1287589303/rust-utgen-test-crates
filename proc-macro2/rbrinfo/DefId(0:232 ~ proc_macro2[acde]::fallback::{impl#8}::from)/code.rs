fn from(inner: TokenStream) -> Self {
        proc_macro::TokenStream::from_str_unchecked(&inner.to_string())
    }