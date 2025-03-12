// Answer 0

#[test]
fn test_search_half_anchored_yes() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };
    
    let haystack = b"example haystack";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::Yes)
        .earliest(true);
    
    let strategy = ReverseAnchored {
        core: Core {
            info: RegexInfo::new(),
            pre: None,
            nfa: NFA::new(),
            nfarev: None,
            pikevm: wrappers::PikeVM::new(),
            backtrack: wrappers::BoundedBacktracker::new(),
            onepass: wrappers::OnePass::new(),
            hybrid: wrappers::Hybrid::new(),
            dfa: wrappers::DFA::new(),
        },
    };

    strategy.search_half(&mut cache, &input);
}

#[test]
fn test_search_half_pattern() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::new(),
        backtrack: wrappers::BoundedBacktrackerCache::new(),
        onepass: wrappers::OnePassCache::new(),
        hybrid: wrappers::HybridCache::new(),
        revhybrid: wrappers::ReverseHybridCache::new(),
    };

    let pattern_id = PatternID::default();
    let haystack = b"another example";
    let input = Input::new(haystack)
        .span(0..haystack.len())
        .anchored(Anchored::Pattern(pattern_id))
        .earliest(false);

    let strategy = ReverseAnchored {
        core: Core {
            info: RegexInfo::new(),
            pre: None,
            nfa: NFA::new(),
            nfarev: None,
            pikevm: wrappers::PikeVM::new(),
            backtrack: wrappers::BoundedBacktracker::new(),
            onepass: wrappers::OnePass::new(),
            hybrid: wrappers::Hybrid::new(),
            dfa: wrappers::DFA::new(),
        },
    };

    strategy.search_half(&mut cache, &input);
}

