pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {
        self.set_span(span);
        self
    }