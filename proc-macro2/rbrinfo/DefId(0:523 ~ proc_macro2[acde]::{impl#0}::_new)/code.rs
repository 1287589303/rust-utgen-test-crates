fn _new(inner: imp::TokenStream) -> Self {
        TokenStream {
            inner,
            _marker: MARKER,
        }
    }