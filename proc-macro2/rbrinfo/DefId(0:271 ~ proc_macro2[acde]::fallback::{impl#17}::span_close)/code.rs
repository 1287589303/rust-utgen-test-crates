pub(crate) fn span_close(&self) -> Span {
        self.span.last_byte()
    }