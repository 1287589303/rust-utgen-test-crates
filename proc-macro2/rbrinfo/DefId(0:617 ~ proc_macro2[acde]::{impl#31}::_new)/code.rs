fn _new(inner: imp::Ident) -> Self {
        Ident {
            inner,
            _marker: MARKER,
        }
    }