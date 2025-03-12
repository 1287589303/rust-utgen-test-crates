// Answer 0

#[test]
fn test_cache_new_valid_dfa() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::always_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let cache = Cache::new(&dfa);
}

#[test]
fn test_cache_new_dfa_with_non_empty_nfa() {
    let nfa = NFA::new("a|b").unwrap();
    let dfa = DFA {
        config: Config::default(),
        nfa,
        stride2: 2,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 20,
    };
    let cache = Cache::new(&dfa);
}

#[test]
fn test_cache_new_dfa_with_min_states() {
    let nfa = NFA::new("abc").unwrap();
    let dfa = DFA {
        config: Config::default(),
        nfa,
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: MIN_STATES,
    };
    let cache = Cache::new(&dfa);
}

