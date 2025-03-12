// Answer 0

#[test]
fn test_which_overlapping_matches_dfa() {
    let input = Input {
        haystack: b"example input",
        span: Span::new(0, 13),
        anchored: Anchored::Yes,
        earliest: true,
    };
    
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None),
        onepass: wrappers::OnePassCache(None),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };
    
    let patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };
    
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI::new())),
        pre: None,
        nfa: NFA(Arc::new(Inner::new())),
        nfarev: Some(NFA(Arc::new(Inner::new()))),
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: DFA::new(&core.info, core.pre, &core.nfa, &core.nfarev.as_ref().unwrap()),
    };

    core.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_hybrid() {
    let input = Input {
        haystack: b"sample data",
        span: Span::new(0, 11),
        anchored: Anchored::No,
        earliest: false,
    };
    
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None),
        onepass: wrappers::OnePassCache(None),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };
    
    let patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };
    
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI::new())),
        pre: None,
        nfa: NFA(Arc::new(Inner::new())),
        nfarev: Some(NFA(Arc::new(Inner::new()))),
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: DFA::new(&core.info, core.pre, &core.nfa, &core.nfarev.as_ref().unwrap()),
    };

    core.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_error() {
    let input = Input {
        haystack: b"error case",
        span: Span::new(0, 10),
        anchored: Anchored::Yes,
        earliest: true,
    };
    
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None),
        onepass: wrappers::OnePassCache(None),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };
    
    let patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([false]), // This should trigger the error
    };
    
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI::new())),
        pre: None,
        nfa: NFA(Arc::new(Inner::new())),
        nfarev: Some(NFA(Arc::new(Inner::new()))),
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: DFA::new(&core.info, core.pre, &core.nfa, &core.nfarev.as_ref().unwrap()),
    };

    core.which_overlapping_matches(&mut cache, &input, &mut patset);
}

