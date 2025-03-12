// Answer 0

#[test]
fn test_search_slots_empty_haystack() {
    let mut cache = BoundedBacktrackerCache(None);
    let input = Input {
        haystack: &[],
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let mut slots = vec![Some(NonMaxUsize::new(1).unwrap())];

    let engine = BoundedBacktrackerEngine(Some(backtrack::BoundedBacktracker::default()));
    engine.search_slots(&mut cache, &input, &mut slots);
}

#[test]
fn test_search_slots_small_haystack() {
    let mut cache = BoundedBacktrackerCache(None);
    let input = Input {
        haystack: b"abc".as_slice(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let mut slots = vec![Some(NonMaxUsize::new(1).unwrap())];

    let engine = BoundedBacktrackerEngine(Some(backtrack::BoundedBacktracker::default()));
    engine.search_slots(&mut cache, &input, &mut slots);
}

#[test]
fn test_search_slots_large_haystack() {
    let mut cache = BoundedBacktrackerCache(None);
    let haystack: Vec<u8> = (0..100).map(|i| i as u8).collect();
    let input = Input {
        haystack: &haystack,
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let mut slots = vec![Some(NonMaxUsize::new(10).unwrap())];

    let engine = BoundedBacktrackerEngine(Some(backtrack::BoundedBacktracker::default()));
    engine.search_slots(&mut cache, &input, &mut slots);
}

#[test]
fn test_search_slots_min_slash() {
    let mut cache = BoundedBacktrackerCache(None);
    let input = Input {
        haystack: b"abc".as_slice(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let mut slots = vec![Some(NonMaxUsize::new(1).unwrap()), None];

    let engine = BoundedBacktrackerEngine(Some(backtrack::BoundedBacktracker::default()));
    engine.search_slots(&mut cache, &input, &mut slots);
}

