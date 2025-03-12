fn from(token: TokenTree) -> Self {
        if inside_proc_macro() {
            TokenStream::Compiler(DeferredTokenStream::new(proc_macro::TokenStream::from(
                into_compiler_token(token),
            )))
        } else {
            TokenStream::Fallback(fallback::TokenStream::from(token))
        }
    }