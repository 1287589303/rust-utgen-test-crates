// Answer 0

#[test]
fn test_start_state_with_valid_look_behind_and_cached_id() {
    let quitset = ByteSet::default();
    let look_behind_byte = 5;
    let config = start::Config {
        look_behind: Some(look_behind_byte),
        anchored: Anchored::No,
    };
    
    let mut cache = Cache {
        trans: vec![],
        starts: vec![LazyStateID(0); 256],
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

    let start_map = StartByteMap { map: [Start::Text; 256] };

    let dfa = DFA {
        config: Config::new(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map,
        classes: ByteClasses([0; 256]),
        quitset: quitset.clone(),
        cache_capacity: 10,
    };

    let lazy_id = LazyStateID(1);
    cache.starts[0] = lazy_id; 

    let result = dfa.start_state(&mut cache, &config);
}

#[test]
fn test_start_state_with_non_empty_quitset() {
    let mut quitset = ByteSet::default();
    quitset.add(10); 
    let look_behind_byte = 5;
    let config = start::Config {
        look_behind: Some(look_behind_byte),
        anchored: Anchored::No,
    };
    
    let mut cache = Cache {
        trans: vec![],
        starts: vec![LazyStateID(0); 256],
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

    let start_map = StartByteMap { map: [Start::Text; 256] };

    let dfa = DFA {
        config: Config::new(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map,
        classes: ByteClasses([0; 256]),
        quitset: quitset.clone(),
        cache_capacity: 10,
    };

    let lazy_id = LazyStateID(1);
    cache.starts[0] = lazy_id;

    let result = dfa.start_state(&mut cache, &config);
}

#[test]
fn test_start_state_with_cache_hit() {
    let mut quitset = ByteSet::default();
    quitset.add(10);
    let look_behind_byte = 5;
    let config = start::Config {
        look_behind: Some(look_behind_byte),
        anchored: Anchored::No,
    };
    
    let mut cache = Cache {
        trans: vec![],
        starts: vec![LazyStateID(1); 256],
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

    let start_map = StartByteMap { map: [Start::Text; 256] };

    let dfa = DFA {
        config: Config::new(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map,
        classes: ByteClasses([0; 256]),
        quitset: quitset.clone(),
        cache_capacity: 10,
    };

    let result = dfa.start_state(&mut cache, &config);
}

