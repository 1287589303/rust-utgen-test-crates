pub(crate) fn new_checked(string: &str, span: Span) -> Self {
        validate_ident(string);
        Ident::new_unchecked(string, span)
    }