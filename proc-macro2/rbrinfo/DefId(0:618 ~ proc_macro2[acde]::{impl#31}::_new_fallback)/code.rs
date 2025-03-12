fn _new_fallback(inner: fallback::Ident) -> Self {
        Ident {
            inner: imp::Ident::from(inner),
            _marker: MARKER,
        }
    }