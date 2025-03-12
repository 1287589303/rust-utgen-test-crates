pub(crate) fn set_span(&mut self, span: Span) {
        match (self, span) {
            (Group::Compiler(g), Span::Compiler(s)) => g.set_span(s),
            (Group::Fallback(g), Span::Fallback(s)) => g.set_span(s),
            (Group::Compiler(_), Span::Fallback(_)) => mismatch(line!()),
            (Group::Fallback(_), Span::Compiler(_)) => mismatch(line!()),
        }
    }