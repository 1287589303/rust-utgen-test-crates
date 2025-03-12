fn unwrap_nightly(self) -> proc_macro::Span {
        match self {
            Span::Compiler(s) => s,
            Span::Fallback(_) => mismatch(line!()),
        }
    }