pub(crate) fn span(&self) -> Span {
        match self {
            Group::Compiler(g) => Span::Compiler(g.span()),
            Group::Fallback(g) => Span::Fallback(g.span()),
        }
    }