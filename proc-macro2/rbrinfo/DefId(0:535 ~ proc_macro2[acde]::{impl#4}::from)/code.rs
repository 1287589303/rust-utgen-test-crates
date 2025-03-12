fn from(inner: TokenStream) -> Self {
        proc_macro::TokenStream::from(inner.inner)
    }