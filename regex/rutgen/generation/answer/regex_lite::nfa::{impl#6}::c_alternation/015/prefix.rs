// Answer 0

#[test]
fn test_c_alternation_empty_iter() {
    struct DummyCompiler {
        nfa: RefCell<NFA>,
    }

    impl DummyCompiler {
        fn new() -> Self {
            Self {
                nfa: RefCell::new(NFA {
                    pattern: String::new(),
                    states: vec![],
                    start: 0,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: None,
                    cap_name_to_index: CaptureNameMap::new(),
                    cap_index_to_name: vec![],
                    memory_extra: 0,
                }),
            }
        }

        fn add_empty(&self) -> Result<StateID, Error> {
            Ok(0)
        }

        fn c_fail(&self) -> Result<ThompsonRef, Error> {
            Ok(ThompsonRef { start: 0, end: 0 })
        }

        fn patch(&self, _from: StateID, _to: StateID) -> Result<(), Error> {
            Ok(())
        }

        fn add(&self, _state: State) -> Result<StateID, Error> {
            Ok(0)
        }

        fn c_alternation<I>(&self, _it: I) -> Result<ThompsonRef, Error>
        where
            I: Iterator<Item = Result<ThompsonRef, Error>>,
        {
            self.c_fail()
        }
    }

    let compiler = DummyCompiler::new();
    let empty_iterator = std::iter::empty::<Result<ThompsonRef, Error>>();
    let _ = compiler.c_alternation(empty_iterator);
}

#[test]
fn test_c_alternation_one_item() {
    struct DummyCompiler {
        nfa: RefCell<NFA>,
    }

    impl DummyCompiler {
        fn new() -> Self {
            Self {
                nfa: RefCell::new(NFA {
                    pattern: String::new(),
                    states: vec![],
                    start: 0,
                    is_start_anchored: false,
                    is_match_empty: false,
                    static_explicit_captures_len: None,
                    cap_name_to_index: CaptureNameMap::new(),
                    cap_index_to_name: vec![],
                    memory_extra: 0,
                }),
            }
        }

        fn add_empty(&self) -> Result<StateID, Error> {
            Ok(0)
        }

        fn c_fail(&self) -> Result<ThompsonRef, Error> {
            Ok(ThompsonRef { start: 0, end: 0 })
        }

        fn patch(&self, _from: StateID, _to: StateID) -> Result<(), Error> {
            Ok(())
        }

        fn add(&self, _state: State) -> Result<StateID, Error> {
            Ok(0)
        }

        fn c_alternation<I>(&self, mut it: I) -> Result<ThompsonRef, Error>
        where
            I: Iterator<Item = Result<ThompsonRef, Error>>,
        {
            let first = match it.next() {
                None => return self.c_fail(),
                Some(result) => result?,
            };
            // Only one item should make it return successfully
            Ok(first)
        }
    }

    let compiler = DummyCompiler::new();
    let single_item_iterator = std::iter::once(Ok(ThompsonRef { start: 1, end: 1 }));
    let _ = compiler.c_alternation(single_item_iterator);
}

