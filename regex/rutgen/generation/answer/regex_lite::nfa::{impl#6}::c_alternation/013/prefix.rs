// Answer 0

#[test]
fn test_c_alternation_with_two_valid_results_and_no_more() {
    struct TestCompiler {
        config: Config,
        nfa: RefCell<NFA>,
    }

    impl TestCompiler {
        fn new(config: Config) -> Self {
            TestCompiler {
                config,
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
            }
        }

        fn add(&self, state: State) -> Result<StateID, Error> {
            let id = u32::try_from(self.nfa.borrow().states.len()).map_err(|_| Error { msg: "exhausted state IDs, too many states" })?;
            self.nfa.borrow_mut().states.push(state);
            Ok(id)
        }

        fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {
            match self.nfa.borrow_mut().states[from.as_usize()] {
                State::Splits { ref mut targets, .. } => {
                    targets.push(to);
                }
                _ => {}
            }
            Ok(())
        }

        fn add_empty(&self) -> Result<StateID, Error> {
            self.add(State::Goto { target: 0, look: None })
        }

        fn c_fail(&self) -> Result<ThompsonRef, Error> {
            let id = self.add(State::Fail)?;
            Ok(ThompsonRef { start: id, end: id })
        }
    }

    let compiler = TestCompiler::new(Config { nest_limit: 10, flags: Flags::empty() });
    
    let first_ref = ThompsonRef { start: 1, end: 2 };
    let second_ref = ThompsonRef { start: 3, end: 4 };
    
    let iterator = vec![
        Ok(first_ref),
        Ok(second_ref),
        Ok(ThompsonRef { start: 5, end: 6 }),
    ].into_iter();

    let result = compiler.c_alternation(iterator);
}

#[test]
fn test_c_alternation_with_invalid_iterator() {
    struct TestCompiler {
        config: Config,
        nfa: RefCell<NFA>,
    }

    impl TestCompiler {
        fn new(config: Config) -> Self {
            TestCompiler {
                config,
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
            }
        }

        fn add(&self, state: State) -> Result<StateID, Error> {
            let id = u32::try_from(self.nfa.borrow().states.len()).map_err(|_| Error { msg: "exhausted state IDs, too many states" })?;
            self.nfa.borrow_mut().states.push(state);
            Ok(id)
        }

        fn patch(&self, from: StateID, to: StateID) -> Result<(), Error> {
            match self.nfa.borrow_mut().states[from.as_usize()] {
                State::Splits { ref mut targets, .. } => {
                    targets.push(to);
                }
                _ => {}
            }
            Ok(())
        }

        fn add_empty(&self) -> Result<StateID, Error> {
            self.add(State::Goto { target: 0, look: None })
        }

        fn c_fail(&self) -> Result<ThompsonRef, Error> {
            let id = self.add(State::Fail)?;
            Ok(ThompsonRef { start: id, end: id })
        }
    }

    let compiler = TestCompiler::new(Config { nest_limit: 10, flags: Flags::empty() });

    let first_ref = ThompsonRef { start: 1, end: 2 };
    let second_ref = ThompsonRef { start: 3, end: 4 };

    let iterator = vec![Ok(first_ref), Ok(second_ref)].into_iter();

    let result = compiler.c_alternation(iterator);
}

