// Answer 0

#[test]
fn test_build_dfa_with_look_state_error() {
    struct NFA {
        look_set_any: LookSet,
        pattern_len: usize,
        group_info: SparseSet,
    }

    let mut nfa = NFA {
        look_set_any: LookSet::full(),
        pattern_len: PatternEpsilons::PATTERN_ID_LIMIT,
        group_info: SparseSet::new(Slots::LIMIT),
    };

    let config = Config::new();

    let mut builder = InternalBuilder {
        dfa: DFA {
            config,
            nfa: nfa.clone(),
            table: vec![],
            starts: vec![],
            min_match_id: StateID::ZERO,
            classes: ByteClasses::default(),
            alphabet_len: 0,
            stride2: 0,
            pateps_offset: 0,
            explicit_slot_start: 0,
        },
        uncompiled_nfa_ids: vec![StateID::new(1).unwrap()],
        nfa_to_dfa_id: vec![StateID::new(1).unwrap()],
        stack: vec![],
        seen: SparseSet::new(64),
        matched: false,
        config: config.clone(),
        nfa: &nfa,
        classes: ByteClasses::default(),
    };

    let _ = builder.nfa.look_set_any().available();
    let _ = builder.nfa.pattern_len().as_u64() == PatternEpsilons::PATTERN_ID_LIMIT;
    let _ = builder.nfa.group_info().explicit_slot_len() == Slots::LIMIT;

    let _ = builder.add_empty_state();
    let _ = builder.add_start_state(None, StateID::ZERO);
    let _ = builder.config.get_starts_for_each_pattern() == false;

    let nfa_id = builder.uncompiled_nfa_ids.pop().unwrap();
    let _ = builder.stack_push(nfa_id, Epsilons::empty());

    while let Some((id, epsilons)) = builder.stack.pop() {
        if let thompson::State::Look { look, next } = &*builder.nfa.state(id) {
            let looks = epsilons.looks().insert(*look);
            let result = builder.stack_push(next, epsilons.set_looks(looks));
            assert!(result.is_err());
        }
    }
}

