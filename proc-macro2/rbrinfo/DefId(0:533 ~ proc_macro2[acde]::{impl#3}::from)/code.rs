fn from(inner: proc_macro::TokenStream) -> Self {
        TokenStream::_new(imp::TokenStream::from(inner))
    }