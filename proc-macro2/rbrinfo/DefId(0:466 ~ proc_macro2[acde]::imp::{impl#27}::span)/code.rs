pub(crate) fn span(&self) -> Span {
        match self {
            Ident::Compiler(t) => Span::Compiler(t.span()),
            Ident::Fallback(t) => Span::Fallback(t.span()),
        }
    }