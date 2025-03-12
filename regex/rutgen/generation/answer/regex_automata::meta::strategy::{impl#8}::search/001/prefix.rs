// Answer 0

#[test]
fn test_search_anchored_yes() {
    let haystack: &[u8] = b"test haystack";
    let pattern_id = PatternID(0);
    let input = Input::new(haystack)
        .anchored(Anchored::Yes)
        .span(0..haystack.len());

    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

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

    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };

    let _ = reverse_suffix.search(&mut cache, &input);
}

#[test]
fn test_search_anchored_pattern() {
    let haystack: &[u8] = b"another test haystack";
    let pattern_id = PatternID(1);
    let input = Input::new(haystack)
        .anchored(Anchored::Pattern(pattern_id))
        .span(0..haystack.len());

    let mut cache = Cache {
        capmatches: Captures::default(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

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

    let reverse_suffix = ReverseSuffix { core, pre: Prefilter::default() };

    let _ = reverse_suffix.search(&mut cache, &input);
}

