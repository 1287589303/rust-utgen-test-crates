fn unwrap_stable(self) -> fallback::TokenStream {
        match self {
            TokenStream::Compiler(_) => mismatch(line!()),
            TokenStream::Fallback(s) => s,
        }
    }