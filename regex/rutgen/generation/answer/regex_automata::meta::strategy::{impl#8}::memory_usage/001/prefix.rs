// Answer 0

#[test]
fn test_memory_usage_zero() {
    let core = Core { 
        info: RegexInfo {}, 
        pre: None, 
        nfa: NFA::new(), 
        nfarev: None, 
        pikevm: wrappers::PikeVM::new(), 
        backtrack: wrappers::BoundedBacktracker::new(), 
        onepass: wrappers::OnePass::new(), 
        hybrid: wrappers::Hybrid::new(), 
        dfa: wrappers::DFA::new() 
    };

    let prefilter = Prefilter { 
        #[cfg(not(feature = "alloc"))]
        _unused: (), 
        #[cfg(feature = "alloc")]
        pre: Arc::new(prefilter::DummyPrefilter::new()), 
        #[cfg(feature = "alloc")]
        is_fast: false, 
        #[cfg(feature = "alloc")]
        max_needle_len: 0 
    };

    let strategy = ReverseSuffix { core, pre: prefilter };
    let _ = strategy.memory_usage();
}

#[test]
fn test_memory_usage_small() {
    let core_memory = 5;
    let pre_memory = 10;
    
    let core = Core { 
        info: RegexInfo {}, 
        pre: Some(Prefilter::new(MatchKind::Anchored, &[]).unwrap()), 
        nfa: NFA::new(), 
        nfarev: None, 
        pikevm: wrappers::PikeVM::new(), 
        backtrack: wrappers::BoundedBacktracker::new(), 
        onepass: wrappers::OnePass::new(), 
        hybrid: wrappers::Hybrid::new(), 
        dfa: wrappers::DFA::new() 
    };

    let prefilter = Prefilter { 
        #[cfg(not(feature = "alloc"))]
        _unused: (), 
        #[cfg(feature = "alloc")]
        pre: Arc::new(prefilter::DummyPrefilter::with_memory(pre_memory)), 
        #[cfg(feature = "alloc")]
        is_fast: false, 
        #[cfg(feature = "alloc")]
        max_needle_len: 5 
    };

    let strategy = ReverseSuffix { core, pre: prefilter };
    let _ = strategy.memory_usage();
}

#[test]
fn test_memory_usage_large() {
    let core_memory = 100;
    let pre_memory = 200;
    
    let core = Core { 
        info: RegexInfo {}, 
        pre: None, 
        nfa: NFA::new(), 
        nfarev: None, 
        pikevm: wrappers::PikeVM::new(), 
        backtrack: wrappers::BoundedBacktracker::new(), 
        onepass: wrappers::OnePass::new(), 
        hybrid: wrappers::Hybrid::new(), 
        dfa: wrappers::DFA::new() 
    };

    let prefilter = Prefilter { 
        #[cfg(not(feature = "alloc"))]
        _unused: (), 
        #[cfg(feature = "alloc")]
        pre: Arc::new(prefilter::DummyPrefilter::with_memory(pre_memory)), 
        #[cfg(feature = "alloc")]
        is_fast: true, 
        #[cfg(feature = "alloc")]
        max_needle_len: 100 
    };

    let strategy = ReverseSuffix { core, pre: prefilter };
    let _ = strategy.memory_usage();
}

