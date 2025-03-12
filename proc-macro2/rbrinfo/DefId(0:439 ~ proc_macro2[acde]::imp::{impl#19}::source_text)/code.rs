pub(crate) fn source_text(&self) -> Option<String> {
        match self {
            #[cfg(not(no_source_text))]
            Span::Compiler(s) => s.source_text(),
            #[cfg(no_source_text)]
            Span::Compiler(_) => None,
            Span::Fallback(s) => s.source_text(),
        }
    }