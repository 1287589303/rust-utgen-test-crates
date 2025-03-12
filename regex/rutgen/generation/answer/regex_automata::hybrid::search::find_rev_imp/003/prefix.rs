// Answer 0

#[test]
fn test_find_rev_imp_empty_input() {
    let dfa = DFA {
        config: Config {},
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 0,
    };

    let mut cache = Cache::new(&dfa);

    let haystack: &[u8] = &[];
    let input = Input::new(haystack)
        .span(Span::new(0, 0))
        .anchored(Anchored::default())
        .earliest(false);

    let result = find_rev_imp(&dfa, &mut cache, &input, true);
}

#[test]
fn test_find_rev_imp_empty_input_anchored() {
    let dfa = DFA {
        config: Config {},
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 0,
    };

    let mut cache = Cache::new(&dfa);

    let haystack: &[u8] = &[];
    let input = Input::new(haystack)
        .span(Span::new(0, 0))
        .anchored(Anchored::Anchored)
        .earliest(true);

    let result = find_rev_imp(&dfa, &mut cache, &input, true);
}

#[test]
fn test_find_rev_imp_empty_input_earliest() {
    let dfa = DFA {
        config: Config {},
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 0,
    };

    let mut cache = Cache::new(&dfa);

    let haystack: &[u8] = &[];
    let input = Input::new(haystack)
        .span(Span::new(0, 0))
        .anchored(Anchored::default())
        .earliest(true);

    let result = find_rev_imp(&dfa, &mut cache, &input, true);
}

#[test]
fn test_find_rev_imp_empty_input_not_earliest() {
    let dfa = DFA {
        config: Config {},
        nfa: thompson::NFA::new(),
        stride2: 0,
        start_map: StartByteMap::new(),
        classes: ByteClasses::new(),
        quitset: ByteSet::new(),
        cache_capacity: 0,
    };

    let mut cache = Cache::new(&dfa);

    let haystack: &[u8] = &[];
    let input = Input::new(haystack)
        .span(Span::new(0, 0))
        .anchored(Anchored::default())
        .earliest(false);

    let result = find_rev_imp(&dfa, &mut cache, &input, false);
}

