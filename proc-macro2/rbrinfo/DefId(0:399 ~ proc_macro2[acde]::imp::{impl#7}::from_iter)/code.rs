fn from_iter<I: IntoIterator<Item = TokenTree>>(trees: I) -> Self {
        if inside_proc_macro() {
            TokenStream::Compiler(DeferredTokenStream::new(
                trees.into_iter().map(into_compiler_token).collect(),
            ))
        } else {
            TokenStream::Fallback(trees.into_iter().collect())
        }
    }