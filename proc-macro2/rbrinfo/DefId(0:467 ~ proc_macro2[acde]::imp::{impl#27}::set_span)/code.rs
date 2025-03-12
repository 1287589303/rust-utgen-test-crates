pub(crate) fn set_span(&mut self, span: Span) {
        match (self, span) {
            (Ident::Compiler(t), Span::Compiler(s)) => t.set_span(s),
            (Ident::Fallback(t), Span::Fallback(s)) => t.set_span(s),
            (Ident::Compiler(_), Span::Fallback(_)) => mismatch(line!()),
            (Ident::Fallback(_), Span::Compiler(_)) => mismatch(line!()),
        }
    }