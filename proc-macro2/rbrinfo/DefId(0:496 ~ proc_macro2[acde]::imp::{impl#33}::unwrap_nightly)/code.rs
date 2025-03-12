fn unwrap_nightly(self) -> proc_macro::Literal {
        match self {
            Literal::Compiler(s) => s,
            Literal::Fallback(_) => mismatch(line!()),
        }
    }