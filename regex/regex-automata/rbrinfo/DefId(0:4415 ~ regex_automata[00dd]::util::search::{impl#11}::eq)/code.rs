fn eq(&self, span: &Span) -> bool {
        self.start == span.start && self.end == span.end
    }