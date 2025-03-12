// Answer 0

#[test]
fn test_search_imp_with_empty_slots_and_no_input() {
    let backtracker = BoundedBacktracker {
        config: Config::new(),
        nfa: NFA::always_match(),
    };

    let input = Input::new(&b""[..])
        .span(Span { start: 0, end: 0 })
        .anchored(Anchored::No);

    let mut cache = Cache::new(&backtracker);
    
    let mut slots: Vec<Option<NonMaxUsize>> = vec![];

    let result = backtracker.search_imp(&mut cache, &input, &mut slots);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_search_imp_with_non_matching_empty_input() {
    let backtracker = BoundedBacktracker {
        config: Config::new(),
        nfa: NFA::never_match(),
    };

    let input = Input::new(&b"abc"[..])
        .span(Span { start: 0, end: 0 })
        .anchored(Anchored::No);

    let mut cache = Cache::new(&backtracker);
    
    let mut slots: Vec<Option<NonMaxUsize>> = vec![];

    let result = backtracker.search_imp(&mut cache, &input, &mut slots);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

