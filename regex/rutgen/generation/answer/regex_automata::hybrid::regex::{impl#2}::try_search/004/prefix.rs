// Answer 0

#[test]
fn test_try_search_forward_no_match() {
    let forward_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let reverse_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let regex = Regex { forward: forward_dfa, reverse: reverse_dfa };
    
    let input_data = b"abcde";
    let input = Input::new(&input_data)
        .span(0..5)
        .anchored(Anchored::No)
        .earliest(false);
    
    let mut cache = Cache {
        forward: dfa::Cache::new(),
        reverse: dfa::Cache::new(),
    };

    let result = regex.try_search(&mut cache, &input);
}

#[test]
fn test_try_search_forward_match_found() {
    let forward_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let reverse_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let regex = Regex { forward: forward_dfa, reverse: reverse_dfa };

    let input_data = b"abcdef";
    let input = Input::new(&input_data)
        .span(0..6)
        .anchored(Anchored::No)
        .earliest(false);

    let mut cache = Cache {
        forward: dfa::Cache::new(),
        reverse: dfa::Cache::new(),
    };

    let result = regex.try_search(&mut cache, &input);
}

#[test]
fn test_try_search_no_reverse_match() {
    let forward_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let reverse_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 8,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 5,
    };

    let regex = Regex { forward: forward_dfa, reverse: reverse_dfa };

    let input_data = b"xyz";
    let input = Input::new(&input_data)
        .span(0..3)
        .anchored(Anchored::No)
        .earliest(false);

    let mut cache = Cache {
        forward: dfa::Cache::new(),
        reverse: dfa::Cache::new(),
    };

    let result = regex.try_search(&mut cache, &input);
}

