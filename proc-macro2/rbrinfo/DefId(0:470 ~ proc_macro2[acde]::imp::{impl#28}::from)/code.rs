fn from(inner: fallback::Ident) -> Self {
        Ident::Fallback(inner)
    }