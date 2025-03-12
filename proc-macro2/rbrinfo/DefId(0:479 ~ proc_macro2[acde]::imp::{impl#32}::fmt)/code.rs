fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ident::Compiler(t) => Debug::fmt(t, f),
            Ident::Fallback(t) => Debug::fmt(t, f),
        }
    }