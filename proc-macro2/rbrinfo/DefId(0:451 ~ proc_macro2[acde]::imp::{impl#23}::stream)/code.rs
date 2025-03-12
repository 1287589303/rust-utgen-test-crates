pub(crate) fn stream(&self) -> TokenStream {
        match self {
            Group::Compiler(g) => TokenStream::Compiler(DeferredTokenStream::new(g.stream())),
            Group::Fallback(g) => TokenStream::Fallback(g.stream()),
        }
    }