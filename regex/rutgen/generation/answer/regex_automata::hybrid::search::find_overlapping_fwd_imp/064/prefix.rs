// Answer 0

#[test]
fn test_find_overlapping_fwd_imp_init_fwd_success_no_matches() {
    let haystack = b"abcde";
    let input = Input::new(&haystack).set_span((0, 5));
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::always_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let mut cache = Cache::new(&dfa);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);

    assert!(result.is_ok());
    assert_eq!(state.at, input.end());
    assert!(state.mat.is_none());
}

#[test]
fn test_find_overlapping_fwd_imp_init_fwd_success_with_matches() {
    let haystack = b"abcab";
    let input = Input::new(&haystack).set_span((0, 5));
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::new("ab").unwrap(), // Assume the pattern is "ab"
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };
    let mut cache = Cache::new(&dfa);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);

    assert!(result.is_ok());
    assert_eq!(state.at, input.end());
    assert!(state.mat.is_none());
}

