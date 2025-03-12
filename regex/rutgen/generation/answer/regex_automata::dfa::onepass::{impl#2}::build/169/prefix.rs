// Answer 0

#[test]
fn test_build_one_pass_dfa() {
    struct MockNFA {
        patterns: usize,
        group_info: usize,
        state: thompson::State,
    }

    impl NFA for MockNFA {
        fn look_set_any(&self) -> LookSet {
            LookSet { bits: 0 }
        }

        fn pattern_len(&self) -> usize {
            PatternEpsilons::PATTERN_ID_LIMIT
        }

        fn group_info(&self) -> usize {
            Slots::LIMIT
        }

        fn start_anchored(&self) -> StateID {
            StateID::new(0).unwrap()
        }

        fn state(&self, id: StateID) -> &thompson::State {
            &self.state
        }
    }

    struct MockConfig {
        starts_for_each_pattern: Option<bool>,
    }

    impl Config for MockConfig {
        fn get_starts_for_each_pattern(&self) -> bool {
            self.starts_for_each_pattern.unwrap_or(false)
        }
    }

    let nfa = MockNFA {
        patterns: 1,
        group_info: 1,
        state: thompson::State::ByteRange {
            trans: Transition {
                start: 0,
                end: 0,
                next: StateID::new(1).unwrap(),
            }
        },
    };

    let config = MockConfig {
        starts_for_each_pattern: None,
    };
    
    let mut builder = InternalBuilder {
        dfa: DFA {
            config,
            nfa,
            table: vec![],
            starts: vec![],
            min_match_id: StateID::new(0).unwrap(),
            classes: ByteClasses([0; 256]),
            alphabet_len: 256,
            stride2: 8,
            pateps_offset: 0,
            explicit_slot_start: 0,
        },
        uncompiled_nfa_ids: vec![StateID::new(0).unwrap()],
        nfa_to_dfa_id: vec![StateID::new(1).unwrap()],
        stack: vec![],
        seen: SparseSet::new(10),
        matched: false,
        config: Config::default(),
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    builder.add_empty_state().unwrap();
    builder.add_start_state(None, builder.dfa.nfa.start_anchored()).unwrap();
    builder.stack_push(builder.uncompiled_nfa_ids.pop().unwrap(), Epsilons::empty()).unwrap();

    while let Some((id, epsilons)) = builder.stack.pop() {
        if let thompson::State::ByteRange { ref trans } = builder.nfa.state(id) {
            builder.compile_transition(StateID::new(0).unwrap(), trans, epsilons).unwrap();
        }
    }

    let result = builder.build();
    assert_eq!(result.is_ok(), true);
}

