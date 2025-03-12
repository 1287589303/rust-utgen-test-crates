// Answer 0

#[test]
fn test_find_overlapping_fwd_imp_case_1() {
    let haystack: &[u8] = b"abcde";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);

    let mut state = OverlappingState {
        mat: None,
        id: Some(LazyStateID::new_unchecked(0)),
        at: 0,
        next_match_index: Some(1),
        rev_eoi: false,
    };

    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::always_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    dfa.next_state_untagged(&mut cache, LazyStateID::new_unchecked(0), b'a');

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);
    assert!(result.is_ok());
}

#[test]
fn test_find_overlapping_fwd_imp_case_2() {
    let haystack: &[u8] = b"fghij";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);

    let mut state = OverlappingState {
        mat: None,
        id: Some(LazyStateID::new_unchecked(1)),
        at: 0,
        next_match_index: Some(1),
        rev_eoi: false,
    };

    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::always_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    dfa.next_state_untagged(&mut cache, LazyStateID::new_unchecked(1), b'f');

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);
    assert!(result.is_ok());
}

#[test]
fn test_find_overlapping_fwd_imp_case_3() {
    let haystack: &[u8] = b"xyzabc";
    let input = Input::new(&haystack)
        .span(Span { start: 0, end: haystack.len() })
        .anchored(Anchored::Yes)
        .earliest(true);

    let mut state = OverlappingState {
        mat: None,
        id: Some(LazyStateID::new_unchecked(2)),
        at: 0,
        next_match_index: Some(2),
        rev_eoi: false,
    };

    let mut dfa = DFA {
        config: Config::default(),
        nfa: NFA::always_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    dfa.next_state_untagged(&mut cache, LazyStateID::new_unchecked(2), b'x');

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);
    assert!(result.is_ok());
}

