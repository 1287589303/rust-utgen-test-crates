// Answer 0

#[test]
fn test_init_fwd_valid_input() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 10,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::default(); 10],
        starts: vec![LazyStateID::default(); 4],
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let input = Input {
        haystack: &[b'a'],
        span: Span::new(0, 1),
        anchored: Anchored::No,
        earliest: true,
    };

    let result = init_fwd(&dfa, &mut cache, &input);
    let sid = result.unwrap();
    assert!(sid.is_match());
}

