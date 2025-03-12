// Answer 0

#[test]
fn test_try_search_full_with_some_span() {
    let prefilter = Prefilter::new(MatchKind::Regex, &["needle"]).unwrap();
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

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"needle haystack"[..]).anchored(Anchored::No).span(0..13);
    let result = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_min_pre_start() {
    let prefilter = Prefilter::new(MatchKind::Regex, &["needle"]).unwrap();
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

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"needle haystack"[..]).anchored(Anchored::No).span(0..13);
    let result = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_with_half_match_none() {
    let prefilter = Prefilter::new(MatchKind::Regex, &["needle"]).unwrap();
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

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"needle haystack"[..]).anchored(Anchored::No).span(0..13);
    let result = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_half_match_found() {
    let prefilter = Prefilter::new(MatchKind::Regex, &["needle"]).unwrap();
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

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"needle haystack"[..]).anchored(Anchored::No).span(0..13);
    let result = reverse_inner.try_search_full(&mut cache, &input);
}

#[test]
fn test_try_search_full_err_case() {
    let prefilter = Prefilter::new(MatchKind::Regex, &["needle"]).unwrap();
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

    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"needle haystack"[..]).anchored(Anchored::No).span(0..13);
    let result = reverse_inner.try_search_full(&mut cache, &input);
}

