// Answer 0

#[test]
fn test_search_half_with_fail_error() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"this is a test string for regex matching";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    reverse_inner.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_with_quadratic_error() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"pattern that may cause quadratic behavior in regex";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    reverse_inner.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_with_multiple_failures() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let haystack = b"invalid test patterns for regex errors";
    let input = Input::new(&haystack)
        .anchored(Anchored::No);

    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let reverse_inner = ReverseInner {
        core,
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    reverse_inner.search_half(&mut cache, &input);
}

