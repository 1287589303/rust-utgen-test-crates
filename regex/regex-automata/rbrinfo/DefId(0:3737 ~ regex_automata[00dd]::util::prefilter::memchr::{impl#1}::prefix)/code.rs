fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        let b = *haystack.get(span.start)?;
        if self.0 == b {
            Some(Span { start: span.start, end: span.start + 1 })
        } else {
            None
        }
    }