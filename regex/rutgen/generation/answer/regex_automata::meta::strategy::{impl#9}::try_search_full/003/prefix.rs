// Answer 0

#[test]
fn test_try_search_full_with_some_span_and_none_half_rev_limited() {
    let haystack: &[u8] = b"example haystack";
    let span = Span { start: 0, end: haystack.len() };
    let anchored = Anchored::Yes;
    let input = Input::new(&haystack).span(span).anchored(anchored);
    let min_pre_start = 0;

    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter::new(MatchKind::All, &[b"example"]).unwrap();
    let core = Core {
        info: RegexInfo::default(),
        pre: Some(prefilter.clone()),
        nfa: NFA::default(),
        nfarev: Some(NFA::default()),
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: prefilter,
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let result = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_with_some_span_and_some_half_rev_limited() {
    let haystack: &[u8] = b"example haystack";
    let span = Span { start: 0, end: haystack.len() };
    let anchored = Anchored::Yes;
    let input = Input::new(&haystack).span(span).anchored(anchored);
    let min_pre_start = 0;

    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let prefilter = Prefilter::new(MatchKind::All, &[b"example"]).unwrap();
    let core = Core {
        info: RegexInfo::default(),
        pre: Some(prefilter.clone()),
        nfa: NFA::default(),
        nfarev: Some(NFA::default()),
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: prefilter,
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let result = reverse_inner.try_search_full(&mut cache, &input);
}

