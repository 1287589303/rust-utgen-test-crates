pub(crate) fn resolved_at(&self, other: Span) -> Span {
        match (self, other) {
            (Span::Compiler(a), Span::Compiler(b)) => Span::Compiler(a.resolved_at(b)),
            (Span::Fallback(a), Span::Fallback(b)) => Span::Fallback(a.resolved_at(b)),
            (Span::Compiler(_), Span::Fallback(_)) => mismatch(line!()),
            (Span::Fallback(_), Span::Compiler(_)) => mismatch(line!()),
        }
    }