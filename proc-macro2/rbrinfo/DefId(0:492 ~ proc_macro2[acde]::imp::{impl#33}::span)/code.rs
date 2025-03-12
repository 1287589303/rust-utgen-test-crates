pub(crate) fn span(&self) -> Span {
        match self {
            Literal::Compiler(lit) => Span::Compiler(lit.span()),
            Literal::Fallback(lit) => Span::Fallback(lit.span()),
        }
    }