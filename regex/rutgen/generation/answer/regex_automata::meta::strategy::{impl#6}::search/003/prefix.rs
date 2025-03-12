// Answer 0

#[test]
fn test_search_no_anchored_with_half_match() {
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input::new(b"example haystack")
        .span(Span::new(0, 15))
        .anchored(Anchored::No)
        .earliest(true);

    let core = Core {
        info: RegexInfo::new(),
        pre: None,
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: wrappers::DFA::new(),
    };

    let strategy = ReverseAnchored { core };

    let result = strategy.search(&mut cache, &input);
}

#[test]
fn test_search_no_anchored_with_no_half_match() {
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input::new(b"another haystack")
        .span(Span::new(0, 15))
        .anchored(Anchored::No)
        .earliest(false);

    let core = Core {
        info: RegexInfo::new(),
        pre: None,
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: wrappers::DFA::new(),
    };

    let strategy = ReverseAnchored { core };

    let result = strategy.search(&mut cache, &input);
}

#[test]
#[should_panic]
fn test_search_no_anchored_with_error() {
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input::new(b"failing haystack")
        .span(Span::new(0, 15))
        .anchored(Anchored::No)
        .earliest(true);

    let core = Core {
        info: RegexInfo::new(),
        pre: None,
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: wrappers::DFA::new(),
    };

    let strategy = ReverseAnchored { core };

    let result = strategy.search(&mut cache, &input);
}

