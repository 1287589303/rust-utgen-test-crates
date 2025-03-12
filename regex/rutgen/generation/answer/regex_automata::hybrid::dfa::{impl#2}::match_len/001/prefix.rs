// Answer 0

#[test]
fn test_match_len_single_pattern() {
    let config = Config::default();
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 4,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 10,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![State::dead()],
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    
    let sid = LazyStateID::new_unchecked(1); // is_match() returns true
    let result = dfa.match_len(&cache, sid);
}

#[test]
fn test_match_len_multiple_patterns() {
    let config = Config::default();
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 4,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 10,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![State::dead()],
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let sid = LazyStateID::new_unchecked(2); // is_match() returns true
    let result = dfa.match_len(&cache, sid);
}

#[test]
#[should_panic]
fn test_match_len_invalid_state() {
    let config = Config::default();
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 4,
        start_map: StartByteMap { map: [Start::default(); 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 10,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID::new_unchecked(0)],
        starts: vec![LazyStateID::new_unchecked(0)],
        states: vec![State::dead()],
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let sid = LazyStateID::new_unchecked(3); // not a match state
    let _ = dfa.match_len(&cache, sid);
}

