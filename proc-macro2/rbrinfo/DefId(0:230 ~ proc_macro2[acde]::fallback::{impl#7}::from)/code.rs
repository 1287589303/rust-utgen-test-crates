fn from(inner: proc_macro::TokenStream) -> Self {
        TokenStream::from_str_unchecked(&inner.to_string())
    }