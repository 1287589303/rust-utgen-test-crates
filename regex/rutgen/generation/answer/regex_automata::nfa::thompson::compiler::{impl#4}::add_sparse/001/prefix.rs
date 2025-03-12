// Answer 0

#[test]
fn test_add_sparse_empty() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: Vec::new(),
            free: Vec::new(),
            iter_stack: RefCell::new(Vec::new()),
            iter_ranges: RefCell::new(Vec::new()),
            dupe_stack: Vec::new(),
            insert_stack: Vec::new(),
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let result = compiler.add_sparse(Vec::new());
}

#[test]
fn test_add_sparse_single_element() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: Vec::new(),
            free: Vec::new(),
            iter_stack: RefCell::new(Vec::new()),
            iter_ranges: RefCell::new(Vec::new()),
            dupe_stack: Vec::new(),
            insert_stack: Vec::new(),
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let transition = Transition { start: 0, end: 255, next: StateID::default() };
    let result = compiler.add_sparse(vec![transition]);
}

#[test]
fn test_add_sparse_multiple_distinct() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: Vec::new(),
            free: Vec::new(),
            iter_stack: RefCell::new(Vec::new()),
            iter_ranges: RefCell::new(Vec::new()),
            dupe_stack: Vec::new(),
            insert_stack: Vec::new(),
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let transitions = vec![
        Transition { start: 1, end: 5, next: StateID::default() },
        Transition { start: 6, end: 10, next: StateID::default() },
        Transition { start: 11, end: 15, next: StateID::default() },
    ];
    let result = compiler.add_sparse(transitions);
}

#[test]
fn test_add_sparse_overlapping_ranges() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: Vec::new(),
            free: Vec::new(),
            iter_stack: RefCell::new(Vec::new()),
            iter_ranges: RefCell::new(Vec::new()),
            dupe_stack: Vec::new(),
            insert_stack: Vec::new(),
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let transitions = vec![
        Transition { start: 0, end: 10, next: StateID::default() },
        Transition { start: 5, end: 15, next: StateID::default() },
    ];
    let result = compiler.add_sparse(transitions);
}

#[test]
fn test_add_sparse_invalid_range() {
    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::new()),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: Vec::new(),
            free: Vec::new(),
            iter_stack: RefCell::new(Vec::new()),
            iter_ranges: RefCell::new(Vec::new()),
            dupe_stack: Vec::new(),
            insert_stack: Vec::new(),
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: Vec::new(),
        }),
    };
    let transition = Transition { start: 10, end: 5, next: StateID::default() };
    let result = compiler.add_sparse(vec![transition]);
}

