pub(crate) fn span_close(&self) -> Span {
        match self {
            Group::Compiler(g) => Span::Compiler(g.span_close()),
            Group::Fallback(g) => Span::Fallback(g.span_close()),
        }
    }