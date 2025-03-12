pub(crate) fn new_raw_checked(string: &str, span: Span) -> Self {
        validate_ident_raw(string);
        Ident::new_raw_unchecked(string, span)
    }