// Answer 0

#[test]
fn test_which_overlapping_matches_empty_cache() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"a".as_ref(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };

    let strategy = ReverseInner {
        core: Core::default(),
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_single_character_input() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"abc".as_ref(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 3,
        which: alloc::boxed::Box::new([true, false, true]),
    };

    let strategy = ReverseInner {
        core: Core::default(),
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_max_byte_input() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack: Vec<u8> = (0..=255).collect();
    let input = Input {
        haystack: &haystack,
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };

    let strategy = ReverseInner {
        core: Core::default(),
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_all_true_pattern_set() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"test input".as_ref(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 5,
        which: alloc::boxed::Box::new([true, true, true, true, true]),
    };

    let strategy = ReverseInner {
        core: Core::default(),
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_mixed_true_false_pattern_set() {
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"sample text".as_ref(),
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 4,
        which: alloc::boxed::Box::new([true, false, true, false]),
    };

    let strategy = ReverseInner {
        core: Core::default(),
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

