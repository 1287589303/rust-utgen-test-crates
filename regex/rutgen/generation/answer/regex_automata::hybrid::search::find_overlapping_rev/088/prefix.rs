// Answer 0

#[test]
fn test_find_overlapping_rev_case1() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 1024,
    };
    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"exampledata";
    let span = Span::new(0, haystack.len());
    let input = Input::new(&haystack).span(span);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 8, // valid index in haystack
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case2() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 1024,
    };
    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"testinput";
    let span = Span::new(0, haystack.len());
    let input = Input::new(&haystack).span(span);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 5, // valid index in haystack
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case3() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 1024,
    };
    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"patternmatch";
    let span = Span::new(0, haystack.len());
    let input = Input::new(&haystack).span(span);
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 10, // valid index in haystack
        next_match_index: None,
        rev_eoi: false,
    };

    let result = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

