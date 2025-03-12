// Answer 0

#[test]
fn test_search_anchored_yes() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let input = Input::new(&b"example"[..])
        .span(0..7)
        .anchored(Anchored::Yes)
        .earliest(true);
    
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

    let _ = reverse_inner.search(&mut cache, &input);
}

#[test]
fn test_search_anchored_pattern() {
    let pattern_id = PatternID::new(1);
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };
    let input = Input::new(&b"example"[..])
        .span(0..7)
        .anchored(Anchored::Pattern(pattern_id))
        .earliest(true);
    
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

    let _ = reverse_inner.search(&mut cache, &input);
}

