fn eq(&self, other: &T) -> bool {
        let other = other.as_ref();
        match self {
            Ident::Compiler(t) => t.to_string() == other,
            Ident::Fallback(t) => t == other,
        }
    }