fn unwrap_nightly(self) -> proc_macro::TokenStream {
        match self {
            TokenStream::Compiler(s) => s.into_token_stream(),
            TokenStream::Fallback(_) => mismatch(line!()),
        }
    }