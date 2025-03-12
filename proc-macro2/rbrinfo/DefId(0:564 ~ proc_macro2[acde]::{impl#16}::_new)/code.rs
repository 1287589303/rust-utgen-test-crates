fn _new(inner: imp::Span) -> Self {
        Span {
            inner,
            _marker: MARKER,
        }
    }