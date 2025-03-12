pub fn new_raw(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_raw_checked(string, span.inner))
    }