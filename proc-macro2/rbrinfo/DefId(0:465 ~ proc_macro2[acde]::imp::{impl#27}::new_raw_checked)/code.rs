pub(crate) fn new_raw_checked(string: &str, span: Span) -> Self {
        match span {
            Span::Compiler(s) => Ident::Compiler(proc_macro::Ident::new_raw(string, s)),
            Span::Fallback(s) => Ident::Fallback(fallback::Ident::new_raw_checked(string, s)),
        }
    }