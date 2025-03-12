// Answer 0

#[test]
fn test_find_overlapping_rev_case_1() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };
    
    let mut cache = Cache::new(&dfa);
    
    let haystack: &[u8] = b"example haystack";
    let input = Input::new(&haystack).span(Span::new(0, 0));
    
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 0,
        next_match_index: None,
        rev_eoi: false,
    };
    
    let _ = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_2() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };
    
    let mut cache = Cache::new(&dfa);
    
    let haystack: &[u8] = b"another example";
    let input = Input::new(&haystack).span(Span::new(3, 3));
    
    let mut state = OverlappingState {
        mat: None,
        id: None,
        at: 3,
        next_match_index: None,
        rev_eoi: false,
    };
    
    let _ = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

#[test]
fn test_find_overlapping_rev_case_3() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 1024,
    };
    
    let mut cache = Cache::new(&dfa);
    
    let haystack: &[u8] = b"test haystack";
    let input = Input::new(&haystack).span(Span::new(4, 4));
    
    let mut state = OverlappingState {
        mat: Some(HalfMatch::new(PatternID::default(), 0)),
        id: None,
        at: 4,
        next_match_index: Some(0),
        rev_eoi: false,
    };
    
    let _ = find_overlapping_rev(&dfa, &mut cache, &input, &mut state);
}

