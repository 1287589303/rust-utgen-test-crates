// Answer 0

#[test]
fn test_try_search_some_match_non_empty_and_anchored_false() {
    let forward_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 10,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 32,
    };
    
    let reverse_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 10,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 32,
    };

    let regex = Regex {
        forward: forward_dfa,
        reverse: reverse_dfa,
    };

    let mut cache = Cache {
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };

    let haystack: &[u8] = b"abcde";
    let input = Input::new(&haystack)
        .span(0..5)
        .anchored(Anchored::No)
        .earliest(false);
    
    let half_match = HalfMatch::new(PatternID(0.into()), 3);
    cache.forward = dfa::Cache::new(Some(half_match));

    // Attempt to perform the search
    regex.try_search(&mut cache, &input).unwrap();
}

#[test]
fn test_try_search_some_match_non_empty_and_overlapping() {
    let forward_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 10,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 32,
    };
    
    let reverse_dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 10,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 32,
    };

    let regex = Regex {
        forward: forward_dfa,
        reverse: reverse_dfa,
    };

    let mut cache = Cache {
        forward: dfa::Cache::default(),
        reverse: dfa::Cache::default(),
    };

    let haystack: &[u8] = b"abcabc";
    let input = Input::new(&haystack)
        .span(1..5)
        .anchored(Anchored::No)
        .earliest(false);
    
    let half_match = HalfMatch::new(PatternID(1.into()), 3);
    cache.forward = dfa::Cache::new(Some(half_match));

    // Attempt to perform the search
    regex.try_search(&mut cache, &input).unwrap();
}

