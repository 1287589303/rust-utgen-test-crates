pub(crate) fn span_open(&self) -> Span {
        match self {
            Group::Compiler(g) => Span::Compiler(g.span_open()),
            Group::Fallback(g) => Span::Fallback(g.span_open()),
        }
    }