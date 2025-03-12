fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {
        (&**self).prefix(haystack, span)
    }