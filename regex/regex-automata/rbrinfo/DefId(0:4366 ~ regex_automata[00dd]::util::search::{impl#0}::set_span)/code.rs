pub fn set_span<S: Into<Span>>(&mut self, span: S) {
        let span = span.into();
        assert!(
            span.end <= self.haystack.len()
                && span.start <= span.end.wrapping_add(1),
            "invalid span {:?} for haystack of length {}",
            span,
            self.haystack.len(),
        );
        self.span = span;
    }