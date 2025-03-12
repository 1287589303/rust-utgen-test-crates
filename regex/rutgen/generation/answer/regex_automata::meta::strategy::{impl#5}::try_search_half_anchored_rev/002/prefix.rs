// Answer 0

#[test]
fn test_try_search_half_anchored_rev_dfa() {
    let core = Core {
        info: RegexInfo::new(),
        pre: None,
        nfa: NFA::new(),
        nfarev: Some(NFA::new()),
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: wrappers::DFA::new(),
    };
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input::new(&b"test haystack"[..])
        .span(Span::new(0, 12))
        .anchored(Anchored::Yes)
        .earliest(true);

    let reverse_anchored = ReverseAnchored::new(core).unwrap();
    reverse_anchored.try_search_half_anchored_rev(&mut cache, &input).unwrap();
}

#[test]
fn test_try_search_half_anchored_rev_hybrid() {
    let core = Core {
        info: RegexInfo::new(),
        pre: None,
        nfa: NFA::new(),
        nfarev: Some(NFA::new()),
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: wrappers::Hybrid::new(),
        dfa: wrappers::DFA::new(),
    };
    
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let input = Input::new(&b"another test haystack"[..])
        .span(Span::new(0, 22))
        .anchored(Anchored::Yes)
        .earliest(false);

    let reverse_anchored = ReverseAnchored::new(core).unwrap();
    reverse_anchored.try_search_half_anchored_rev(&mut cache, &input).unwrap();
}

