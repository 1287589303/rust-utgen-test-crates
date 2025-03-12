// Answer 0

#[test]
fn test_search_non_anchored_quadratic_error() {
    let haystack: &[u8] = b"simple test input";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(false);

    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let core = Core {
        info: core_info,
        pre: Some(prefilter),
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner::new(core, &[]).unwrap();

    let _ = reverse_inner.search(&mut cache, &input);
}

#[test]
fn test_search_non_anchored_fail_error() {
    let haystack: &[u8] = b"another test input";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(true);

    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let core = Core {
        info: core_info,
        pre: Some(prefilter),
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner::new(core, &[]).unwrap();

    let _ = reverse_inner.search(&mut cache, &input);
}

#[test]
fn test_search_non_anchored_success() {
    let haystack: &[u8] = b"successful match case";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack)
        .span(span)
        .anchored(Anchored::No)
        .earliest(false);
    
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let core_info = RegexInfo::default();
    let prefilter = Prefilter::default();
    let core = Core {
        info: core_info,
        pre: Some(prefilter),
        nfa: NFA::new(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner::new(core, &[]).unwrap();

    let _ = reverse_inner.search(&mut cache, &input);
}

