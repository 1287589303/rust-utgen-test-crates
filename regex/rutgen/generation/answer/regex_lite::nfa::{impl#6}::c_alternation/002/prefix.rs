// Answer 0

#[test]
fn test_c_alternation_with_two_ok_results_then_err() {
    struct TestIterator {
        state: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.state {
                0 => {
                    self.state += 1;
                    Some(Ok(ThompsonRef { start: 1, end: 2 }))
                }
                1 => {
                    self.state += 1;
                    Some(Ok(ThompsonRef { start: 3, end: 4 }))
                }
                _ => {
                    self.state += 1;
                    Some(Err(Error { msg: "Iterator ended" }))
                }
            }
        }
    }

    let compiler = Compiler {
        config: Config { nest_limit: 10, flags: Default::default() },
        nfa: RefCell::new(NFA {
            pattern: String::new(),
            states: Vec::new(),
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: Vec::new(),
            memory_extra: 0,
        }),
    };

    let iterator = TestIterator { state: 0 };

    let _result = compiler.c_alternation(iterator);
}

#[test]
fn test_c_alternation_with_multiple_ok_results() {
    struct TestIterator {
        state: usize,
    }

    impl Iterator for TestIterator {
        type Item = Result<ThompsonRef, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.state {
                0 => {
                    self.state += 1;
                    Some(Ok(ThompsonRef { start: 1, end: 2 }))
                }
                1 => {
                    self.state += 1;
                    Some(Ok(ThompsonRef { start: 3, end: 4 }))
                }
                2 => {
                    self.state += 1;
                    Some(Ok(ThompsonRef { start: 5, end: 6 }))
                }
                _ => None,
            }
        }
    }

    let compiler = Compiler {
        config: Config { nest_limit: 10, flags: Default::default() },
        nfa: RefCell::new(NFA {
            pattern: String::new(),
            states: Vec::new(),
            start: 0,
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
            cap_name_to_index: CaptureNameMap::new(),
            cap_index_to_name: Vec::new(),
            memory_extra: 0,
        }),
    };

    let iterator = TestIterator { state: 0 };

    let _result = compiler.c_alternation(iterator);
}

