// Answer 0

#[test]
fn test_c_alt_iter_with_successive_items() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 2 {
                self.count += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(0),
                    end: StateID(1),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::SomeError }))
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let iter = TestIterator { count: 0 };
    let _ = compiler.c_alt_iter(iter);
}

#[test]
fn test_c_alt_iter_with_multiple_successes() {
    struct TestIterator {
        count: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, BuildError>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 3 {
                self.count += 1;
                Some(Ok(ThompsonRef {
                    start: StateID(self.count as u32),
                    end: StateID(self.count as u32 + 1),
                }))
            } else {
                Some(Err(BuildError { kind: BuildErrorKind::SomeError }))
            }
        }
    }

    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder::default()),
        utf8_state: RefCell::new(Utf8State::default()),
        trie_state: RefCell::new(RangeTrie::default()),
        utf8_suffix: RefCell::new(Utf8SuffixMap::default()),
    };

    let iter = TestIterator { count: 0 };
    let _ = compiler.c_alt_iter(iter);
}

