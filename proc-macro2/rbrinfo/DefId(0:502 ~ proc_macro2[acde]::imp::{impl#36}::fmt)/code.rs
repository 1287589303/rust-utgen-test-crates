fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Compiler(t) => Debug::fmt(t, f),
            Literal::Fallback(t) => Debug::fmt(t, f),
        }
    }