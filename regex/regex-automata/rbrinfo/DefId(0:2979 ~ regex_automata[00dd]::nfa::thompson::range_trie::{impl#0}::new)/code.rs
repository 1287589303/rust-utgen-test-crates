pub fn new() -> RangeTrie {
        let mut trie = RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        };
        trie.clear();
        trie
    }