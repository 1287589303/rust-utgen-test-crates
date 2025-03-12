fn from(proc_span: proc_macro::Span) -> Self {
        crate::Span::_new(Span::Compiler(proc_span))
    }