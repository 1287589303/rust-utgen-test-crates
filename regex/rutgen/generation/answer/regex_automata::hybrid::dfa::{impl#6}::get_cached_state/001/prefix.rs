// Answer 0

#[test]
fn test_get_cached_state_valid_index() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 10,
    };
    let cache = Cache {
        states: vec![State(Arc::from(vec![0; 10]))],
        // omitted other Cache fields for brevity
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };
    let lazy_ref = LazyRef { dfa: &dfa, cache: &cache };
    let lazy_id = LazyStateID::new_unchecked(0);
    let state = lazy_ref.get_cached_state(lazy_id);
}

#[test]
fn test_get_cached_state_boundary_index() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 2,
        start_map: StartByteMap::default(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 10,
    };
    let cache = Cache {
        states: vec![State(Arc::from(vec![0; 20]))],
        // omitted other Cache fields for brevity
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };
    let lazy_ref = LazyRef { dfa: &dfa, cache: &cache };
    let lazy_id = LazyStateID::new_unchecked(3); // (3 >> 2) = 0
    let state = lazy_ref.get_cached_state(lazy_id);
}

#[test]
#[should_panic]
fn test_get_cached_state_invalid_index() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 10,
    };
    let cache = Cache {
        states: vec![State(Arc::from(vec![0; 5]))],
        // omitted other Cache fields for brevity
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };
    let lazy_ref = LazyRef { dfa: &dfa, cache: &cache };
    let lazy_id = LazyStateID::new_unchecked(16); // (16 >> 1) = 8, which is out of bounds
    let state = lazy_ref.get_cached_state(lazy_id);
}

