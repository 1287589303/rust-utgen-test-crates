// Answer 0

#[test]
fn test_build_dfa_success() {
    let nfa = NFA::always_match(); // An NFA that always matches
    let config = Config::new()
        .match_kind(MatchKind::LeftmostLongest)
        .starts_for_each_pattern(true)
        .byte_classes(true);
    
    let mut builder = InternalBuilder {
        dfa: DFA::new(config.clone(), &nfa),
        uncompiled_nfa_ids: vec![StateID::ZERO],
        nfa_to_dfa_id: vec![StateID::ZERO; nfa.pattern_len()],
        stack: vec![],
        seen: SparseSet::new(32), 
        matched: false,
        config: config,
        nfa: &nfa,
        classes: ByteClasses::default(),
    };
    
    builder.add_empty_state().unwrap();
    builder.add_start_state(None, nfa.start_anchored()).unwrap();

    for pid in nfa.patterns() {
        builder.add_start_state(Some(pid), nfa.start_pattern(pid).unwrap()).unwrap();
    }

    while let Some(nfa_id) = builder.uncompiled_nfa_ids.pop() {
        builder.stack_push(nfa_id, Epsilons::empty()).unwrap();
        while let Some((id, epsilons)) = builder.stack.pop() {
            match *nfa.state(id) {
                thompson::State::Sparse(ref sparse) => {
                    for trans in sparse.transitions.iter() {
                        builder.compile_transition(builder.dfa_id, trans, epsilons).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    let result = builder.build();
    assert!(result.is_ok());
}

#[test]
fn test_build_dfa_pattern_limit() {
    let nfa = NFA::new_many(&["pattern1", "pattern2"]).unwrap();
    let config = Config::new()
        .match_kind(MatchKind::LeftmostLongest)
        .starts_for_each_pattern(true);
    
    let mut builder = InternalBuilder {
        dfa: DFA::new(config.clone(), &nfa),
        uncompiled_nfa_ids: vec![StateID::ZERO],
        nfa_to_dfa_id: vec![StateID::ZERO; nfa.pattern_len()],
        stack: vec![],
        seen: SparseSet::new(32),
        matched: false,
        config,
        nfa: &nfa,
        classes: ByteClasses::default(),
    };

    builder.add_empty_state().unwrap();
    builder.add_start_state(None, nfa.start_anchored()).unwrap();

    for pid in nfa.patterns() {
        builder.add_start_state(Some(pid), nfa.start_pattern(pid).unwrap()).unwrap();
    }

    while let Some(nfa_id) = builder.uncompiled_nfa_ids.pop() {
        builder.stack_push(nfa_id, Epsilons::empty()).unwrap();
        while let Some((id, epsilons)) = builder.stack.pop() {
            match *nfa.state(id) {
                thompson::State::Sparse(ref sparse) => {
                    for trans in sparse.transitions.iter() {
                        builder.compile_transition(builder.dfa_id, trans, epsilons).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    let result = builder.build();
    assert!(result.is_ok());
}

#[test]
fn test_build_dfa_with_slots_limit() {
    let nfa = NFA::new_many(&["pattern_with_slots"]).unwrap();
    
    let mut config = Config::new()
        .match_kind(MatchKind::LeftmostLongest)
        .starts_for_each_pattern(true);
    
    config.make_slot_limit(Slots::LIMIT);
    
    let mut builder = InternalBuilder {
        dfa: DFA::new(config.clone(), &nfa),
        uncompiled_nfa_ids: vec![StateID::ZERO],
        nfa_to_dfa_id: vec![StateID::ZERO; nfa.pattern_len()],
        stack: vec![],
        seen: SparseSet::new(32),
        matched: false,
        config,
        nfa: &nfa,
        classes: ByteClasses::default(),
    };

    builder.add_empty_state().unwrap();
    builder.add_start_state(None, nfa.start_anchored()).unwrap();

    for pid in nfa.patterns() {
        builder.add_start_state(Some(pid), nfa.start_pattern(pid).unwrap()).unwrap();
    }

    while let Some(nfa_id) = builder.uncompiled_nfa_ids.pop() {
        builder.stack_push(nfa_id, Epsilons::empty()).unwrap();
        while let Some((id, epsilons)) = builder.stack.pop() {
            match *nfa.state(id) {
                thompson::State::Sparse(ref sparse) => {
                    for trans in sparse.transitions.iter() {
                        builder.compile_transition(builder.dfa_id, trans, epsilons).unwrap();
                    }
                }
                _ => {}
            }
        }
    }

    let result = builder.build();
    assert!(result.is_ok());
}

