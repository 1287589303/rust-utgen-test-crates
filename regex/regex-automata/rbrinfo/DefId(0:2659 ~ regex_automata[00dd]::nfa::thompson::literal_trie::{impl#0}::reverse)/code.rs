pub(crate) fn reverse() -> LiteralTrie {
        let root = State::default();
        LiteralTrie { states: vec![root], rev: true }
    }