// Answer 0

#[test]
fn test_try_which_overlapping_matches_min_input() {
    let input = Input {
        haystack: &[b'a'],
        span: Span { start: 0, end: 1 },
        anchored: Anchored(true),
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };
    let engine = DFAEngine::new(&RegexInfo::default(), None, &NFA::default(), &NFA::default()).unwrap();
    let _ = engine.try_which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_try_which_overlapping_matches_max_haystack() {
    let input = Input {
        haystack: &[b'a'; 4096],
        span: Span { start: 0, end: 4096 },
        anchored: Anchored(false),
        earliest: false,
    };
    let mut patset = PatternSet {
        len: 2,
        which: alloc::boxed::Box::new([true, false]),
    };
    let engine = DFAEngine::new(&RegexInfo::default(), None, &NFA::default(), &NFA::default()).unwrap();
    let _ = engine.try_which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_try_which_overlapping_matches_varied_anchored() {
    let input = Input {
        haystack: &[b'a', b'b', b'c', b'd'],
        span: Span { start: 0, end: 4 },
        anchored: Anchored(true),
        earliest: false,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };
    let engine = DFAEngine::new(&RegexInfo::default(), None, &NFA::default(), &NFA::default()).unwrap();
    let _ = engine.try_which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_try_which_overlapping_matches_multiple_patterns() {
    let input = Input {
        haystack: &[b'a', b'b', b'a', b'b'],
        span: Span { start: 0, end: 4 },
        anchored: Anchored(false),
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 3,
        which: alloc::boxed::Box::new([true, true, false]),
    };
    let engine = DFAEngine::new(&RegexInfo::default(), None, &NFA::default(), &NFA::default()).unwrap();
    let _ = engine.try_which_overlapping_matches(&input, &mut patset);
}

#[test]
fn test_try_which_overlapping_matches_edge_case_empty_pattern() {
    let input = Input {
        haystack: &[b'a', b'b', b'a', b'b'],
        span: Span { start: 0, end: 4 },
        anchored: Anchored(false),
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([false]),
    };
    let engine = DFAEngine::new(&RegexInfo::default(), None, &NFA::default(), &NFA::default()).unwrap();
    let _ = engine.try_which_overlapping_matches(&input, &mut patset);
}

