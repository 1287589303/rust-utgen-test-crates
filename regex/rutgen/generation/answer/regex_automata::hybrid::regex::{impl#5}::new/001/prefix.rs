// Answer 0

#[test]
fn test_cache_new_valid_regex() {
    let dfa = dfa::DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
    };

    let regex = Regex {
        forward: dfa.clone(),
        reverse: dfa,
    };

    let cache = Cache::new(&regex);
}

#[test]
fn test_cache_new_empty_regex() {
    let dfa = dfa::DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
    };

    let regex = Regex {
        forward: dfa.clone(),
        reverse: dfa,
    };

    let cache = Cache::new(&regex);
}

#[test]
fn test_cache_new_complex_regex() {
    let dfa = dfa::DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
    };

    let regex = Regex {
        forward: dfa.clone(),
        reverse: dfa,
    };

    let cache = Cache::new(&regex);
}

