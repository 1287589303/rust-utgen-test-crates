// Answer 0

#[test]
fn test_c_concat_reverse_some_none() {
    struct TestIterator {
        current: usize,
        end: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.end {
                self.current += 1;
                Some(Ok(ThompsonRef { start: StateID(0), end: StateID(1) }))
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.current > 0 {
                self.current -= 1;
                Some(Ok(ThompsonRef { start: StateID(0), end: StateID(1) }))
            } else {
                None
            }
        }
    }

    let iterator = TestIterator { current: 0, end: 1 };

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { look_behind: Some(1), anchored: Anchored::No },
        builder: RefCell::new(Builder { config: Config::default() }),
        utf8_state: RefCell::new(Utf8State { compiled: Utf8BoundedMap::default(), uncompiled: Vec::new() }),
        trie_state: RefCell::new(RangeTrie { states: Vec::new(), free: Vec::new(), iter_stack: RefCell::new(Vec::new()), iter_ranges: RefCell::new(Vec::new()), dupe_stack: Vec::new(), insert_stack: Vec::new() }),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: Vec::new() }),
    };

    let _result = compiler.c_concat(iterator);
}

