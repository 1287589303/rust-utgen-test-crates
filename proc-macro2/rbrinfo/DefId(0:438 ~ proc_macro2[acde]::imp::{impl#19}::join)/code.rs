pub(crate) fn join(&self, other: Span) -> Option<Span> {
        let ret = match (self, other) {
            #[cfg(proc_macro_span)]
            (Span::Compiler(a), Span::Compiler(b)) => Span::Compiler(a.join(b)?),
            (Span::Fallback(a), Span::Fallback(b)) => Span::Fallback(a.join(b)?),
            _ => return None,
        };
        Some(ret)
    }