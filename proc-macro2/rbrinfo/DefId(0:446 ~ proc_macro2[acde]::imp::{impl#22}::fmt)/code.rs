fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Span::Compiler(s) => Debug::fmt(s, f),
            Span::Fallback(s) => Debug::fmt(s, f),
        }
    }