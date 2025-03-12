pub(crate) fn new() -> Self {
        if inside_proc_macro() {
            TokenStream::Compiler(DeferredTokenStream::new(proc_macro::TokenStream::new()))
        } else {
            TokenStream::Fallback(fallback::TokenStream::new())
        }
    }