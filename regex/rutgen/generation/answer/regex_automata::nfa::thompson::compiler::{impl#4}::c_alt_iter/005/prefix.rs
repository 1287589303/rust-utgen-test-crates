// Answer 0

#[test]
fn test_c_alt_iter_successful_case() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 2 {
                self.current += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(0),
                    end: StateID(1),
                }))
            } else {
                None
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 10,
            map: Vec::new(),
        }),
    };

    let it = TestIterator { current: 0 };
    let _ = compiler.c_alt_iter(it);
}

#[test]
fn test_c_alt_iter_multiple_cases() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < 3 {
                self.current += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(self.current as u32),
                    end: StateID(self.current as u32 + 1),
                }))
            } else {
                None
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 10,
            map: Vec::new(),
        }),
    };

    let it = TestIterator { current: 0 };
    let _ = compiler.c_alt_iter(it);
}

#[test]
fn test_c_alt_iter_with_empty_case() {
    struct TestIterator {
        current: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current == 0 {
                self.current += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(0),
                    end: StateID(1),
                }))
            } else if self.current == 1 {
                self.current += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(1),
                    end: StateID(2),
                }))
            } else {
                None
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 10,
            map: Vec::new(),
        }),
    };

    let it = TestIterator { current: 0 };
    let _ = compiler.c_alt_iter(it);
}

