// Answer 0

#[test]
fn test_try_clear_cache_bad_efficiency() {
    let mut cache = Cache {
        clear_count: 1,
        memory_usage_state: 0,
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        bytes_searched: 1,
        progress: None,
    };
    
    let config = Config {
        minimum_cache_clear_count: Some(1),
        minimum_bytes_per_state: Some(1),
        ..Default::default()
    };
    
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
    };
    
    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache
    };

    let result = lazy.try_clear_cache();

    // Result should be Err(CacheError::bad_efficiency())
}

#[test]
fn test_try_clear_cache_bad_efficiency_boundary() {
    let mut cache = Cache {
        clear_count: 2,
        memory_usage_state: 0,
        trans: Vec::new(),
        starts: Vec::new(),
        states: Vec::new(),
        states_to_id: StateMap::new(),
        sparses: SparseSets::default(),
        stack: Vec::new(),
        scratch_state_builder: StateBuilderEmpty::default(),
        state_saver: StateSaver::default(),
        bytes_searched: 0,
        progress: None,
    };
    
    let config = Config {
        minimum_cache_clear_count: Some(2),
        minimum_bytes_per_state: Some(1),
        ..Default::default()
    };
    
    let dfa = DFA {
        config,
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
    };
    
    let mut lazy = Lazy {
        dfa: &dfa,
        cache: &mut cache
    };

    let result = lazy.try_clear_cache();

    // Result should still be Err(CacheError::bad_efficiency())
}

