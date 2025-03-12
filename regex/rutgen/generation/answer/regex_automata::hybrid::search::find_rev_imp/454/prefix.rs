// Answer 0

#[test]
fn test_find_rev_imp_non_empty_haystack() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"abcd";
    let input = Input::new(&haystack).span(Span::new(0, haystack.len() as usize));

    let result = find_rev_imp(&dfa, &mut cache, &input, false);
}

#[test]
fn test_find_rev_imp_varied_search() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"abcdefgh";
    let input = Input::new(&haystack).span(Span::new(0, haystack.len() as usize));

    let result = find_rev_imp(&dfa, &mut cache, &input, true);
}

#[test]
fn test_find_rev_imp_with_assumptions() {
    let dfa = DFA {
        config: Config::default(),
        nfa: thompson::NFA::default(),
        stride2: 1,
        start_map: StartByteMap::default(),
        classes: ByteClasses::default(),
        quitset: ByteSet::default(),
        cache_capacity: 10,
    };

    let mut cache = Cache::new(&dfa);
    let haystack: &[u8] = b"abcde";
    let input = Input::new(&haystack).span(Span::new(1, 4));

    let mut sid = LazyStateID::new_unchecked(0); // valid untagged state
    let result = find_rev_imp(&dfa, &mut cache, &input, false);
}

