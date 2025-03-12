fn unwrap_nightly(self) -> proc_macro::Ident {
        match self {
            Ident::Compiler(s) => s,
            Ident::Fallback(_) => mismatch(line!()),
        }
    }