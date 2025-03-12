pub(crate) fn call_site() -> Self {
        LexError {
            span: Span::call_site(),
        }
    }