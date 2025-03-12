fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        (&**self).find(haystack, span)
    }