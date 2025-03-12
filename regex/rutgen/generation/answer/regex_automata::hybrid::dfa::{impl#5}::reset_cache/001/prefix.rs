// Answer 0

#[test]
fn test_reset_cache_with_initialized_dfa_and_cache() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 10],
        starts: vec![LazyStateID::default(); 4],
        states: vec![State::ByteRange { trans: Transition::default() }; 5],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(10),
        memory_usage_state: 0,
        clear_count: 1,
        bytes_searched: 0,
        progress: None,
    };
    let nfa = NFA::new("a|b").unwrap();
    let dfa = DFA {
        config: Config::default(),
        nfa: nfa.clone(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
        table: vec![Transition::default(); 20],
        starts: vec![StateID::default(); 1],
        min_match_id: StateID::default(),
        alphabet_len: 2,
    };
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    lazy.reset_cache();
}

#[test]
fn test_reset_cache_with_empty_nfa() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 0],
        starts: vec![LazyStateID::default(); 0],
        states: vec![],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(0),
        memory_usage_state: 0,
        clear_count: 0,
        bytes_searched: 0,
        progress: None,
    };
    let nfa = NFA::never_match();
    let dfa = DFA {
        config: Config::default(),
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1,
        table: vec![],
        starts: vec![],
        min_match_id: StateID::default(),
        alphabet_len: 0,
    };
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    lazy.reset_cache();
}

#[test]
fn test_reset_cache_with_large_nfa() {
    let mut cache = Cache {
        stack: Vec::new(),
        curr: ActiveStates::default(),
        next: ActiveStates::default(),
        trans: vec![LazyStateID::default(); 50],
        starts: vec![LazyStateID::default(); 5],
        states: vec![State::ByteRange { trans: Transition::default() }; 40],
        states_to_id: StateMap::new(),
        sparses: SparseSets::new(50),
        memory_usage_state: 0,
        clear_count: 5,
        bytes_searched: 100,
        progress: Some(SearchProgress { start: 0, at: 50 }),
    };
    let nfa = NFA::new("abc|def|ghi").unwrap();
    let dfa = DFA {
        config: Config::default(),
        nfa,
        stride2: 16,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
        table: vec![Transition::default(); 80],
        starts: vec![StateID::default(); 10],
        min_match_id: StateID::default(),
        alphabet_len: 3,
    };
    let mut lazy = Lazy { dfa: &dfa, cache: &mut cache };
    lazy.reset_cache();
}

