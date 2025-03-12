fn as_ref<'a>(&'a self) -> LazyRef<'i, 'a> {
        LazyRef::new(self.dfa, self.cache)
    }