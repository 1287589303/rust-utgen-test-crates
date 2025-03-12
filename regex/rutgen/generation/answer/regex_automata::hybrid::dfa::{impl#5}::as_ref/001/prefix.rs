// Answer 0

#[test]
fn test_as_ref_valid() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };
    let mut cache = Cache {
        trans: vec![LazyStateID::new(0); 10],
        starts: vec![LazyStateID::new(0)],
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

    let lazy = Lazy::new(&dfa, &mut cache);
    let lazy_ref = lazy.as_ref();
}

#[test]
fn test_as_ref_empty_cache() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 4,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 0,
    };
    let mut cache = Cache {
        trans: vec![],
        starts: vec![],
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

    let lazy = Lazy::new(&dfa, &mut cache);
    let lazy_ref = lazy.as_ref();
}

#[test]
fn test_as_ref_large_cache() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 16,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 2048,
    };
    let mut cache = Cache {
        trans: vec![LazyStateID::new(0); 50],
        starts: vec![LazyStateID::new(0), LazyStateID::new(1)],
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

    let lazy = Lazy::new(&dfa, &mut cache);
    let lazy_ref = lazy.as_ref();
}

