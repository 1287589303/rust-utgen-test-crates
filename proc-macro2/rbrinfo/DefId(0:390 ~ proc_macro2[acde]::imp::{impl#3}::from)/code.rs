fn from(inner: proc_macro::TokenStream) -> Self {
        TokenStream::Compiler(DeferredTokenStream::new(inner))
    }