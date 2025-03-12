pub(crate) fn new_unchecked(string: &str, span: Span) -> Self {
        Ident {
            sym: Box::from(string),
            span,
            raw: false,
        }
    }