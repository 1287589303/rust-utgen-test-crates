fn from(inner: fallback::Span) -> Self {
        Span::Fallback(inner)
    }