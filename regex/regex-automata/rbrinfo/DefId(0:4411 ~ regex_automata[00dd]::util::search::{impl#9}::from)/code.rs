fn from(span: Span) -> Range<usize> {
        Range { start: span.start, end: span.end }
    }