// Answer 0

#[test]
fn test_find_overlapping_fwd_imp_init_fwd_err() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::never_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"example";
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 7 })
        .anchored(Anchored::Unanchored)
        .earliest(false);

    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);
}

#[test]
fn test_find_overlapping_fwd_imp_starting_state_none() {
    let dfa = DFA {
        config: Config::default(),
        nfa: NFA::never_match(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 100,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"example";
    let input = Input::new(haystack)
        .span(Span { start: 0, end: 7 })
        .anchored(Anchored::Unanchored)
        .earliest(false);

    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_fwd_imp(&dfa, &mut cache, &input, None, &mut state);
}

