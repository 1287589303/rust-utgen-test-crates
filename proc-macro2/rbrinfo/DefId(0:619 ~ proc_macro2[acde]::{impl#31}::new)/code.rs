pub fn new(string: &str, span: Span) -> Self {
        Ident::_new(imp::Ident::new_checked(string, span.inner))
    }