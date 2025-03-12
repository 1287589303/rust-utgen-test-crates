// Answer 0

#[test]
fn test_hybrid_try_search_half_rev_case_1() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };

    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![],
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let input = Input::new(&[b'a']).set_span(Span::new(0, 0)).set_earliest(true);
    let min_start = 0;

    let _result = hybrid_try_search_half_rev(&dfa, &mut cache, &input, min_start);
}

#[test]
fn test_hybrid_try_search_half_rev_case_2() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };

    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(1)],
        states: vec![],
        states_to_id: StateMap::default(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let input = Input::new(&[b'b']).set_span(Span::new(0, 0)).set_earliest(true);
    let min_start = 0;

    let _result = hybrid_try_search_half_rev(&dfa, &mut cache, &input, min_start);
}

