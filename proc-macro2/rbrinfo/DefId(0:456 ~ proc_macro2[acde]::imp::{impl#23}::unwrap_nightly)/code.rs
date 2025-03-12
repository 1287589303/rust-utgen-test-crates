fn unwrap_nightly(self) -> proc_macro::Group {
        match self {
            Group::Compiler(g) => g,
            Group::Fallback(_) => mismatch(line!()),
        }
    }