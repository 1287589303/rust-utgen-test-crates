// Answer 0

#[test]
fn test_next_state_untagged_case_tagged_current() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [0; 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID(0); SENTINEL_STATES],
        starts: vec![LazyStateID(0); SENTINEL_STATES],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let current = LazyStateID(1); // tagged state ID
    let input: u8 = 65; // valid byte

    let _result = dfa.next_state_untagged(&cache, current, input);
}

#[test]
#[should_panic]
fn test_next_state_untagged_case_invalid_current() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap { map: [0; 256] },
        classes: ByteClasses([0; 256]),
        quitset: ByteSet([false; 256]),
        cache_capacity: 0,
    };
    
    let mut cache = Cache {
        trans: vec![LazyStateID(0); SENTINEL_STATES],
        starts: vec![LazyStateID(0); SENTINEL_STATES],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: vec![],
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };

    let current = LazyStateID(2); // invalid as assumed untagged
    let input: u8 = 100; // valid byte

    let _result = dfa.next_state_untagged(&cache, current, input);
}

