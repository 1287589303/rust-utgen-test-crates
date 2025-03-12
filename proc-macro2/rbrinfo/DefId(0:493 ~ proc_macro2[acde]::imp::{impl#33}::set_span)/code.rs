pub(crate) fn set_span(&mut self, span: Span) {
        match (self, span) {
            (Literal::Compiler(lit), Span::Compiler(s)) => lit.set_span(s),
            (Literal::Fallback(lit), Span::Fallback(s)) => lit.set_span(s),
            (Literal::Compiler(_), Span::Fallback(_)) => mismatch(line!()),
            (Literal::Fallback(_), Span::Compiler(_)) => mismatch(line!()),
        }
    }