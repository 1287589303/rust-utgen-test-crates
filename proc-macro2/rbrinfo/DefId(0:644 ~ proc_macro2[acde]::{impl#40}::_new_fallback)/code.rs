fn _new_fallback(inner: fallback::Literal) -> Self {
        Literal {
            inner: imp::Literal::from(inner),
            _marker: MARKER,
        }
    }