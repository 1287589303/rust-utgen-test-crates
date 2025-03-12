pub(crate) fn call_site() -> Self {
        if inside_proc_macro() {
            Span::Compiler(proc_macro::Span::call_site())
        } else {
            Span::Fallback(fallback::Span::call_site())
        }
    }