// Answer 0

#[test]
fn test_search_slots_unanchored_capture_search_not_needed_none() {
    let slots: &mut [Option<NonMaxUsize>] = &mut [None; 10];
    let haystack: &[u8] = b"some haystack data";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::No);
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let reverse_inner = ReverseInner {
        core: Core {
            info: RegexInfo::default(),
            pre: None,
            nfa: NFA::default(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    reverse_inner.search_slots(&mut cache, &input, slots);
}

#[test]
fn test_search_slots_unanchored_capture_search_not_needed_err() {
    let slots: &mut [Option<NonMaxUsize>] = &mut [None; 10];
    let haystack: &[u8] = b"example input";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::No);
    let cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let reverse_inner = ReverseInner {
        core: Core {
            info: RegexInfo::default(),
            pre: None,
            nfa: NFA::default(),
            nfarev: None,
            pikevm: wrappers::PikeVM::default(),
            backtrack: wrappers::BoundedBacktracker::default(),
            onepass: wrappers::OnePass::default(),
            hybrid: wrappers::Hybrid::default(),
            dfa: wrappers::DFA::default(),
        },
        preinner: Prefilter::default(),
        nfarev: NFA::default(),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    reverse_inner.search_slots(&mut cache, &input, slots);
}

