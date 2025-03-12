fn new(dfa: &'i DFA, cache: &'c Cache) -> LazyRef<'i, 'c> {
        LazyRef { dfa, cache }
    }