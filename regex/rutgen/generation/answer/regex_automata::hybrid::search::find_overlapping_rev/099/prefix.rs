// Answer 0

#[test]
fn test_find_overlapping_rev_case_1() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"abcde";
    let input = Input::new(&haystack)
        .span(Span::new(0, 5));
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 4,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_2() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"testing";
    let input = Input::new(&haystack)
        .span(Span::new(0, 7));
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 6,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_3() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 0,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"example";
    let input = Input::new(&haystack)
        .span(Span::new(0, 7));
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 6,
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

