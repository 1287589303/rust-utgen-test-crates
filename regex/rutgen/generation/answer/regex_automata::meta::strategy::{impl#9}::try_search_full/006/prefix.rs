// Answer 0

#[test]
fn test_try_search_full_case_1() {
    let haystack: &[u8] = b"example haystack";
    let span = Span { start: 0, end: 0 };
    let anchored_mode = Anchored::No;

    let input = Input::new(haystack).span(span).anchored(anchored_mode);
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let mut prefilter = Prefilter::new(MatchKind::Simple, &[b"sample"]);
    let core = Core {
        info: RegexInfo::default(),
        pre: Some(prefilter.clone()),
        nfa: NFA(Arc::new(Inner::default())),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: prefilter,
        nfarev: NFA(Arc::new(Inner::default())),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let _ = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_case_2() {
    let haystack: &[u8] = b"another example";
    let span = Span { start: 0, end: 0 };
    let anchored_mode = Anchored::No;

    let input = Input::new(haystack).span(span).anchored(anchored_mode);
    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter::new(MatchKind::Simple, &[b"example"]);
    let core = Core {
        info: RegexInfo::default(),
        pre: Some(prefilter.clone()),
        nfa: NFA(Arc::new(Inner::default())),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: prefilter,
        nfarev: NFA(Arc::new(Inner::default())),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let _ = reverse_inner.try_search_full(&mut cache, &input);
}

