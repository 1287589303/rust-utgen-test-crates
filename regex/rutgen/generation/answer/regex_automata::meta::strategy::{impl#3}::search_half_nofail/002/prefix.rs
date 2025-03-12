// Answer 0

#[test]
fn test_search_half_nofail_valid_case() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"test input",
        span: Span::from(0..10),
        anchored: Anchored::No,
        earliest: false,
    };

    let strategy = Core {
        info: RegexInfo(Arc::new(RegexInfoI::default())),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    assert!(strategy.search_half_nofail(&mut cache, &input).is_some());
}

#[test]
fn test_search_half_nofail_with_anchored_input() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"another test",
        span: Span::from(0..12),
        anchored: Anchored::Yes,
        earliest: true,
    };

    let strategy = Core {
        info: RegexInfo(Arc::new(RegexInfoI::default())),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    assert!(strategy.search_half_nofail(&mut cache, &input).is_some());
}

#[test]
fn test_search_half_nofail_earliest_case() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input {
        haystack: b"searching test",
        span: Span::from(0..15),
        anchored: Anchored::No,
        earliest: true,
    };

    let strategy = Core {
        info: RegexInfo(Arc::new(RegexInfoI::default())),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    assert!(strategy.search_half_nofail(&mut cache, &input).is_some());
}

