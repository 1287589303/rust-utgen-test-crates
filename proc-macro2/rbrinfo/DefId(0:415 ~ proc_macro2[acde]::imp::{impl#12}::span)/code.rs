pub(crate) fn span(&self) -> Span {
        match self {
            LexError::Compiler(_) | LexError::CompilerPanic => Span::call_site(),
            LexError::Fallback(e) => Span::Fallback(e.span()),
        }
    }