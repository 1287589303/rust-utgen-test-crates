fn eq(&self, other: &Ident) -> bool {
        match (self, other) {
            (Ident::Compiler(t), Ident::Compiler(o)) => t.to_string() == o.to_string(),
            (Ident::Fallback(t), Ident::Fallback(o)) => t == o,
            (Ident::Compiler(_), Ident::Fallback(_)) => mismatch(line!()),
            (Ident::Fallback(_), Ident::Compiler(_)) => mismatch(line!()),
        }
    }