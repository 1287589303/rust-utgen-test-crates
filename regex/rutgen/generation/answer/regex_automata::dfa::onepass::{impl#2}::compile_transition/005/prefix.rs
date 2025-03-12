// Answer 0

#[test]
fn test_compile_transition_success_no_bytes() {
    struct DummyNFA;
    impl DummyNFA {
        fn new() -> Self {
            DummyNFA
        }
    }

    let nfa = DummyNFA::new();
    let classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config {},
        nfa: nfa.clone(),
        stride2: 9,
        start_map: StartByteMap {},
        classes,
        quitset: ByteSet {},
        cache_capacity: 0,
    };

    let mut builder = InternalBuilder {
        dfa,
        uncompiled_nfa_ids: vec![],
        nfa_to_dfa_id: vec![DEAD; 256],
        stack: vec![],
        seen: SparseSet::default(),
        matched: false,
        config: Config::default(),
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    let dfa_id = StateID::new_unchecked(0);
    let trans = thompson::Transition {
        start: 0,
        end: 0,
        next: StateID::new_unchecked(1),
    };
    let epsilons = Epsilons(0);

    builder.add_dfa_state_for_nfa_state(StateID::new_unchecked(1)).unwrap();

    let result = builder.compile_transition(dfa_id, &trans, epsilons);
}

#[test]
fn test_compile_transition_success_no_existing_transition() {
    struct DummyNFA;
    impl DummyNFA {
        fn new() -> Self {
            DummyNFA
        }
    }

    let nfa = DummyNFA::new();
    let classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config {},
        nfa: nfa.clone(),
        stride2: 9,
        start_map: StartByteMap {},
        classes,
        quitset: ByteSet {},
        cache_capacity: 0,
    };

    let mut builder = InternalBuilder {
        dfa,
        uncompiled_nfa_ids: vec![],
        nfa_to_dfa_id: vec![DEAD; 256],
        stack: vec![],
        seen: SparseSet::default(),
        matched: false,
        config: Config::default(),
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    let dfa_id = StateID::new_unchecked(0);
    let trans = thompson::Transition {
        start: 1,
        end: 1,
        next: StateID::new_unchecked(2),
    };
    let epsilons = Epsilons(0);

    builder.add_dfa_state_for_nfa_state(StateID::new_unchecked(2)).unwrap();

    let result = builder.compile_transition(dfa_id, &trans, epsilons);
}

#[test]
fn test_compile_transition_with_valid_epsilons() {
    struct DummyNFA;
    impl DummyNFA {
        fn new() -> Self {
            DummyNFA
        }
    }

    let nfa = DummyNFA::new();
    let classes = ByteClasses([0; 256]);
    let dfa = DFA {
        config: Config {},
        nfa: nfa.clone(),
        stride2: 9,
        start_map: StartByteMap {},
        classes,
        quitset: ByteSet {},
        cache_capacity: 0,
    };

    let mut builder = InternalBuilder {
        dfa,
        uncompiled_nfa_ids: vec![],
        nfa_to_dfa_id: vec![DEAD; 256],
        stack: vec![],
        seen: SparseSet::default(),
        matched: false,
        config: Config::default(),
        nfa: &nfa,
        classes: ByteClasses([0; 256]),
    };

    let dfa_id = StateID::new_unchecked(2);
    let trans = thompson::Transition {
        start: 0,
        end: 255,
        next: StateID::new_unchecked(3),
    };
    let epsilons = Epsilons(1);

    builder.add_dfa_state_for_nfa_state(StateID::new_unchecked(3)).unwrap();

    let result = builder.compile_transition(dfa_id, &trans, epsilons);
}

