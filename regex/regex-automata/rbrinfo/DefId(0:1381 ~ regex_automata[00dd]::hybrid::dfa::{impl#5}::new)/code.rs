fn new(dfa: &'i DFA, cache: &'c mut Cache) -> Lazy<'i, 'c> {
        Lazy { dfa, cache }
    }