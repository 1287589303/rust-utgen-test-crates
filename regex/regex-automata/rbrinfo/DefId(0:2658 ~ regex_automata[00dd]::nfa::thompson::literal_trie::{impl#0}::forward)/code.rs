pub(crate) fn forward() -> LiteralTrie {
        let root = State::default();
        LiteralTrie { states: vec![root], rev: false }
    }