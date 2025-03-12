// Answer 0

#[test]
fn test_is_valid_multiple_of_stride_zero() {
    let transitions = vec![LazyStateID(0), LazyStateID(2), LazyStateID(4)];
    let dfa = DFA {
        config: Default::default(),
        nfa: Default::default(),
        stride2: 1,
        start_map: Default::default(),
        classes: Default::default(),
        quitset: Default::default(),
        cache_capacity: 0,
    };
    let cache = Cache {
        trans: transitions.clone(),
        starts: Default::default(),
        states: Default::default(),
        states_to_id: Default::default(),
        sparses: Default::default(),
        stack: Default::default(),
        scratch_state_builder: Default::default(),
        state_saver: Default::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Default::default(),
    };

    let lazy_ref = LazyRef {
        dfa: &dfa,
        cache: &cache,
    };

    let id = LazyStateID::new_unchecked(0); // Valid index and multiple of stride
    lazy_ref.is_valid(id);
}

#[test]
fn test_is_valid_multiple_of_stride_non_zero() {
    let transitions = vec![LazyStateID(0), LazyStateID(1), LazyStateID(2), LazyStateID(3), LazyStateID(4)];
    let dfa = DFA {
        config: Default::default(),
        nfa: Default::default(),
        stride2: 2,
        start_map: Default::default(),
        classes: Default::default(),
        quitset: Default::default(),
        cache_capacity: 0,
    };
    let cache = Cache {
        trans: transitions.clone(),
        starts: Default::default(),
        states: Default::default(),
        states_to_id: Default::default(),
        sparses: Default::default(),
        stack: Default::default(),
        scratch_state_builder: Default::default(),
        state_saver: Default::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Default::default(),
    };

    let lazy_ref = LazyRef {
        dfa: &dfa,
        cache: &cache,
    };

    let id = LazyStateID::new_unchecked(4); // Valid index and multiple of stride
    lazy_ref.is_valid(id);
}

#[test]
fn test_is_valid_boundary_case() {
    let transitions = vec![LazyStateID(0), LazyStateID(1), LazyStateID(2)];
    let dfa = DFA {
        config: Default::default(),
        nfa: Default::default(),
        stride2: 1,
        start_map: Default::default(),
        classes: Default::default(),
        quitset: Default::default(),
        cache_capacity: 0,
    };
    let cache = Cache {
        trans: transitions.clone(),
        starts: Default::default(),
        states: Default::default(),
        states_to_id: Default::default(),
        sparses: Default::default(),
        stack: Default::default(),
        scratch_state_builder: Default::default(),
        state_saver: Default::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: Default::default(),
    };

    let lazy_ref = LazyRef {
        dfa: &dfa,
        cache: &cache,
    };

    let id = LazyStateID::new_unchecked(2); // Valid index and multiple of stride
    lazy_ref.is_valid(id);
}

