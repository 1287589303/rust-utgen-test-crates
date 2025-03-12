// Answer 0

#[test]
fn test_is_match_with_fail_error() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"simple test string"[..])
        .anchored(Anchored::No)
        .span(0..17)
        .earliest(false);

    let strategy = ReverseInner {
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

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_with_quadratic_error() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"another simple case"[..])
        .anchored(Anchored::No)
        .span(0..20)
        .earliest(false);

    let strategy = ReverseInner {
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

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_with_ok_none() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"empty input"[..])
        .anchored(Anchored::No)
        .span(0..12)
        .earliest(false);

    let strategy = ReverseInner {
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

    strategy.is_match(&mut cache, &input);
}

#[test]
fn test_is_match_with_ok_some() {
    let cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache::default(),
        backtrack: wrappers::BoundedBacktrackerCache::default(),
        onepass: wrappers::OnePassCache::default(),
        hybrid: wrappers::HybridCache::default(),
        revhybrid: wrappers::ReverseHybridCache::default(),
    };

    let input = Input::new(&b"matched pattern"[..])
        .anchored(Anchored::No)
        .span(0..15)
        .earliest(false);

    let strategy = ReverseInner {
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

    strategy.is_match(&mut cache, &input);
}

