pub fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match {
        let span: Span = span.into();
        assert!(span.start <= span.end, "invalid match span");
        Match { pattern, span }
    }