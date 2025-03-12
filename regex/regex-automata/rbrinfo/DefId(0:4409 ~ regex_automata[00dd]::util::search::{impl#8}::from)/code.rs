fn from(range: Range<usize>) -> Span {
        Span { start: range.start, end: range.end }
    }