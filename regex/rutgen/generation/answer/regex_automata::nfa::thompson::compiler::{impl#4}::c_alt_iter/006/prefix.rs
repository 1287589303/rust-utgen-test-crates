// Answer 0

#[test]
fn test_c_alt_iter_with_successful_first_and_second() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(Ok(ThompsonRef { start: StateID(0), end: StateID(1) }))
            } else if self.count == 1 {
                self.count += 1;
                Some(Ok(ThompsonRef { start: StateID(2), end: StateID(3) }))
            } else {
                None
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let mut it = TestIterator { count: 0 };

    let _ = compiler.c_alt_iter(&mut it);
}

#[test]
fn test_c_alt_iter_with_success_and_failure_on_patch() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count == 0 {
                self.count += 1;
                Some(Ok(ThompsonRef { start: StateID(4), end: StateID(5) }))
            } else if self.count == 1 {
                self.count += 1;
                Some(Ok(ThompsonRef { start: StateID(6), end: StateID(7) }))
            } else {
                None
            }
        }
    }

    let mut add_union_called = false;
    let mut add_empty_called = false;

    let mut compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
            // used for mock and will panic to simulate failure on patch
            dfa: dfa::Builder::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let mut it = TestIterator { count: 0 };

    let _ = compiler.c_alt_iter(&mut it);
}

