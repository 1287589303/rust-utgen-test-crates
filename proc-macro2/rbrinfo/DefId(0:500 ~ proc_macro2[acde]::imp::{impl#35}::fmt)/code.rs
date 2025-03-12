fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Compiler(t) => Display::fmt(t, f),
            Literal::Fallback(t) => Display::fmt(t, f),
        }
    }