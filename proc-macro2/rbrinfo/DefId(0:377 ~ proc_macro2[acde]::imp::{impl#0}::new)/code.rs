fn new(stream: proc_macro::TokenStream) -> Self {
        DeferredTokenStream {
            stream,
            extra: Vec::new(),
        }
    }