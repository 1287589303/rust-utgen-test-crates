// Answer 0

#[test]
fn test_add_builder_state_with_cached_id() {
    let mut cache = Cache {
        states_to_id: std::collections::HashMap::new(),
        ..Default::default()
    };
    let builder = StateBuilderNFA {
        repr: vec![1, 2, 3],
        prev_nfa_state_id: 0,
    };
    let cached_id = LazyStateID(42);
    cache.states_to_id.insert(builder.as_bytes().to_vec(), cached_id);

    let mut dfa = DFA {
        cache,
        ..Default::default()
    };

    let idmap = |id: LazyStateID| id;

    let _ = dfa.add_builder_state(builder.clone(), idmap);
}

#[test]
fn test_add_builder_state_with_non_empty_representation() {
    let mut cache = Cache {
        states_to_id: std::collections::HashMap::new(),
        ..Default::default()
    };
    let builder = StateBuilderNFA {
        repr: vec![4, 5, 6],
        prev_nfa_state_id: 0,
    };
    let cached_id = LazyStateID(84);
    cache.states_to_id.insert(builder.as_bytes().to_vec(), cached_id);

    let mut dfa = DFA {
        cache,
        ..Default::default()
    };

    let idmap = |id: LazyStateID| id;

    let _ = dfa.add_builder_state(builder.clone(), idmap);
}

#[test]
fn test_add_builder_state_with_different_bytes() {
    let mut cache = Cache {
        states_to_id: std::collections::HashMap::new(),
        ..Default::default()
    };
    let builder = StateBuilderNFA {
        repr: vec![7, 8, 9],
        prev_nfa_state_id: 0,
    };
    let cached_id = LazyStateID(99);
    cache.states_to_id.insert(builder.as_bytes().to_vec(), cached_id);

    let mut dfa = DFA {
        cache,
        ..Default::default()
    };

    let idmap = |id: LazyStateID| id;

    let _ = dfa.add_builder_state(builder.clone(), idmap);
}

