fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ident::Compiler(t) => Display::fmt(t, f),
            Ident::Fallback(t) => Display::fmt(t, f),
        }
    }