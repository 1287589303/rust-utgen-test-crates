pub(crate) fn span_open(&self) -> Span {
        self.span.first_byte()
    }