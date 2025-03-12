// Answer 0

#[test]
fn test_c_alt_iter_one_item() {
    struct TestIterator {
        called: bool,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if !self.called {
                self.called = true;
                Some(Ok(ThompsonRef { start: StateID(0), end: StateID(1) }))
            } else {
                None
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config {
            match_kind: MatchKind::default(),
            quit: ByteSet::default(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        },
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
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

    let iterator = TestIterator { called: false };
    let _ = compiler.c_alt_iter(iterator);
}

