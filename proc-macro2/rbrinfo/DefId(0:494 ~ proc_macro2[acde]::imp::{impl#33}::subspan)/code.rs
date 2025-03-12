pub(crate) fn subspan<R: RangeBounds<usize>>(&self, range: R) -> Option<Span> {
        match self {
            #[cfg(proc_macro_span)]
            Literal::Compiler(lit) => lit.subspan(range).map(Span::Compiler),
            #[cfg(not(proc_macro_span))]
            Literal::Compiler(_lit) => None,
            Literal::Fallback(lit) => lit.subspan(range).map(Span::Fallback),
        }
    }