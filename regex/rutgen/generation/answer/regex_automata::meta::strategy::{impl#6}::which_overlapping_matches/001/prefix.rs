// Answer 0

#[test]
fn test_which_overlapping_matches_non_empty_haystack() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    
    let input = Input {
        haystack: b"test string for matching",
        span: Span::default(),
        anchored: Anchored::Yes,
        earliest: true,
    };
    
    let patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::from([true]),
    };
    
    let strategy = ReverseAnchored {
        core: Core {
            info: RegexInfo::default(),
            pre: None,
            nfa: NFA::new(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_empty_input() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    
    let input = Input {
        haystack: b"",
        span: Span::default(),
        anchored: Anchored::Yes,
        earliest: true,
    };
    
    let patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::from([true]),
    };
    
    let strategy = ReverseAnchored {
        core: Core {
            info: RegexInfo::default(),
            pre: None,
            nfa: NFA::new(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_multiple_patterns() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    
    let input = Input {
        haystack: b"another test string for overlapping",
        span: Span::default(),
        anchored: Anchored::Yes,
        earliest: true,
    };
    
    let patset = PatternSet {
        len: 3,
        which: alloc::boxed::Box::from([true, false, true]),
    };
    
    let strategy = ReverseAnchored {
        core: Core {
            info: RegexInfo::default(),
            pre: None,
            nfa: NFA::new(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
    };

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

