pub(crate) fn new_checked(string: &str, span: Span) -> Self {
        match span {
            Span::Compiler(s) => Ident::Compiler(proc_macro::Ident::new(string, s)),
            Span::Fallback(s) => Ident::Fallback(fallback::Ident::new_checked(string, s)),
        }
    }