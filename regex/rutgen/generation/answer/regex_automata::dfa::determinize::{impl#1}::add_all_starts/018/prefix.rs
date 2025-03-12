// Answer 0

#[test]
fn test_add_all_starts_no_patterns() {
    let config = Config { 
        match_kind: None,
        utf8_empty: None,
        autopre: None,
        pre: None,
        which_captures: None,
        nfa_size_limit: None,
        onepass_size_limit: None,
        hybrid_cache_capacity: None,
        hybrid: None,
        dfa: Some(true),
        dfa_size_limit: None,
        dfa_state_limit: None,
        onepass: None,
        backtrack: None,
        byte_classes: None,
        line_terminator: None,
    };

    struct TestNFA {
        patterns_len: usize,
    }

    impl TestNFA {
        fn patterns(&self) -> PatternIter<'_> {
            PatternIter {
                it: PatternID::iter(self.patterns_len),
                _marker: core::marker::PhantomData,
            }
        }

        fn start_anchored(&self) -> StateID {
            StateID::default()
        }

        fn start_unanchored(&self) -> StateID {
            StateID::default()
        }

        fn start_pattern(&self, _pid: PatternID) -> Option<StateID> {
            None
        }

        fn has_empty(&self) -> bool {
            false
        }
    }

    struct TestDFA {
        start_kind: StartKind,
        starts_for_each_pattern: bool,
    }

    impl TestDFA {
        fn start_kind(&self) -> &StartKind {
            &self.start_kind
        }

        fn starts_for_each_pattern(&self) -> bool {
            self.starts_for_each_pattern
        }
    }

    let nfa = TestNFA { patterns_len: 0 };
    let dfa = TestDFA {
        start_kind: StartKind::Both,
        starts_for_each_pattern: true,
    };

    let mut dfa_state_ids = Vec::new();
    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: Vec::new(),
        cache: StateMap::default(),
        memory_usage_state: 0,
        sparses: SparseSets {
            set1: SparseSet::default(),
            set2: SparseSet::default(),
        },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
    };

    let result = runner.add_all_starts(&mut dfa_state_ids);
    assert!(result.is_ok());
}

#[test]
fn test_add_all_starts_patterns_present() {
    let config = Config { 
        match_kind: None,
        utf8_empty: None,
        autopre: None,
        pre: None,
        which_captures: None,
        nfa_size_limit: None,
        onepass_size_limit: None,
        hybrid_cache_capacity: None,
        hybrid: None,
        dfa: Some(true),
        dfa_size_limit: None,
        dfa_state_limit: None,
        onepass: None,
        backtrack: None,
        byte_classes: None,
        line_terminator: None,
    };

    struct TestNFA {
        patterns_len: usize,
    }

    impl TestNFA {
        fn patterns(&self) -> PatternIter<'_> {
            PatternIter {
                it: PatternID::iter(self.patterns_len),
                _marker: core::marker::PhantomData,
            }
        }

        fn start_anchored(&self) -> StateID {
            StateID::default()
        }

        fn start_unanchored(&self) -> StateID {
            StateID::default()
        }

        fn start_pattern(&self, _pid: PatternID) -> Option<StateID> {
            None
        }

        fn has_empty(&self) -> bool {
            false
        }
    }

    struct TestDFA {
        start_kind: StartKind,
        starts_for_each_pattern: bool,
    }

    impl TestDFA {
        fn start_kind(&self) -> &StartKind {
            &self.start_kind
        }

        fn starts_for_each_pattern(&self) -> bool {
            self.starts_for_each_pattern
        }
    }

    let nfa = TestNFA { patterns_len: 1 };
    let dfa = TestDFA {
        start_kind: StartKind::Both,
        starts_for_each_pattern: true,
    };

    let mut dfa_state_ids = Vec::new();
    let mut runner = Runner {
        config,
        nfa: &nfa,
        dfa: &mut dfa,
        builder_states: Vec::new(),
        cache: StateMap::default(),
        memory_usage_state: 0,
        sparses: SparseSets {
            set1: SparseSet::default(),
            set2: SparseSet::default(),
        },
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty(Vec::new()),
    };

    let result = runner.add_all_starts(&mut dfa_state_ids);
    assert!(result.is_ok());
}

