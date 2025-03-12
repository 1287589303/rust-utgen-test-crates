pub(crate) fn mixed_site() -> Self {
        if inside_proc_macro() {
            Span::Compiler(proc_macro::Span::mixed_site())
        } else {
            Span::Fallback(fallback::Span::mixed_site())
        }
    }