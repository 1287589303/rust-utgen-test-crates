fn _new(inner: imp::Literal) -> Self {
        Literal {
            inner,
            _marker: MARKER,
        }
    }