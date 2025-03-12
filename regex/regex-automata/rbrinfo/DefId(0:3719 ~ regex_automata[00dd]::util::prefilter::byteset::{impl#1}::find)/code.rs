fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        haystack[span].iter().position(|&b| self.0[usize::from(b)]).map(|i| {
            let start = span.start + i;
            let end = start + 1;
            Span { start, end }
        })
    }