// Answer 0

#[test]
fn test_c_concat_some_then_err() {
    struct TestIterator {
        state: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.state < 2 {
                self.state += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(1),
                    end: StateID(2),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::new() }))
            }
        }
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.state < 2 {
                self.state += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(1),
                    end: StateID(2),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::new() }))
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { look_behind: None, anchored: Anchored::Yes },
        builder: RefCell::new(Builder { config: Config::default() }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };

    let iter = TestIterator { state: 0 };
    let _ = compiler.c_concat(iter);
}

#[test]
fn test_c_concat_non_empty() {
    struct TestIterator {
        state: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.state < 2 {
                self.state += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(1),
                    end: StateID(2),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::new() }))
            }
        }
    }

    impl DoubleEndedIterator for TestIterator {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.state < 2 {
                self.state += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(1),
                    end: StateID(2),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::new() }))
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::new(),
        config: Config { look_behind: None, anchored: Anchored::Yes },
        builder: RefCell::new(Builder { config: Config::default() }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: vec![],
        }),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap { version: 0, capacity: 0, map: vec![] }),
    };

    let iter = TestIterator { state: 0 };
    let _ = compiler.c_concat(iter);
}

