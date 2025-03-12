pub(crate) fn unwrap(self) -> proc_macro::Span {
        match self {
            Span::Compiler(s) => s,
            Span::Fallback(_) => panic!("proc_macro::Span is only available in procedural macros"),
        }
    }