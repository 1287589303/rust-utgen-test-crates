fn from(inner: fallback::TokenStream) -> Self {
        TokenStream::Fallback(inner)
    }