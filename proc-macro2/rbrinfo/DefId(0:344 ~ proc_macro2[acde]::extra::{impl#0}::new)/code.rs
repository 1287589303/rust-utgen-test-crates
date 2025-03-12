pub(crate) fn new(group: &imp::Group) -> Self {
        #[cfg(wrap_proc_macro)]
        let inner = match group {
            imp::Group::Compiler(group) => DelimSpanEnum::Compiler {
                join: group.span(),
                open: group.span_open(),
                close: group.span_close(),
            },
            imp::Group::Fallback(group) => DelimSpanEnum::Fallback(group.span()),
        };

        #[cfg(not(wrap_proc_macro))]
        let inner = DelimSpanEnum::Fallback(group.span());

        DelimSpan {
            inner,
            _marker: MARKER,
        }
    }