// Answer 0

#[test]
fn test_which_overlapping_matches_dfa() {
    let info = RegexInfo(Arc::new(RegexInfoI {}));
    let nfa = NFA(Arc::new(Inner {}));
    let nfarev = NFA(Arc::new(Inner {}));
    let dfa = DFA::new(&info, None, &nfa, &nfarev);
    let hybrid = Hybrid::new(&info, None, &nfa, &nfarev);
    
    let mut cache = Cache { 
        capmatches: Captures {}, 
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None), 
        onepass: wrappers::OnePassCache(None), 
        hybrid: wrappers::HybridCache(Some(hybrid::regex::Cache {})), 
        revhybrid: wrappers::ReverseHybridCache {}, 
    };

    let input = Input {
        haystack: b"abcde",
        span: Span {},
        anchored: Anchored {},
        earliest: true,
    };

    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };

    let strategy = Core { 
        info, 
        pre: None, 
        nfa: nfa.clone(), 
        nfarev: Some(nfarev), 
        pikevm: wrappers::PikeVM {}, 
        backtrack: wrappers::BoundedBacktracker { config: Config {}, nfa: nfa.clone() }, 
        onepass: wrappers::OnePass {}, 
        hybrid 
    };

    strategy.dfa = Some(dfa);
    strategy.hybrid = Some(hybrid);

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_matches_hybrid() {
    let info = RegexInfo(Arc::new(RegexInfoI {}));
    let nfa = NFA(Arc::new(Inner {}));
    let nfarev = NFA(Arc::new(Inner {}));
    let dfa = DFA::new(&info, None, &nfa, &nfarev);
    let hybrid = Hybrid::new(&info, None, &nfa, &nfarev);
    
    let mut cache = Cache { 
        capmatches: Captures {}, 
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None), 
        onepass: wrappers::OnePassCache(None), 
        hybrid: wrappers::HybridCache(Some(hybrid::regex::Cache {})), 
        revhybrid: wrappers::ReverseHybridCache {}, 
    };

    let input = Input {
        haystack: b"abcde",
        span: Span {},
        anchored: Anchored {},
        earliest: false,
    };

    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true]),
    };

    let strategy = Core { 
        info, 
        pre: None, 
        nfa: nfa.clone(), 
        nfarev: Some(nfarev), 
        pikevm: wrappers::PikeVM {}, 
        backtrack: wrappers::BoundedBacktracker { config: Config {}, nfa: nfa.clone() }, 
        onepass: wrappers::OnePass {}, 
        hybrid 
    };

    strategy.dfa = Some(dfa);
    strategy.hybrid = Some(hybrid);

    strategy.which_overlapping_matches(&mut cache, &input, &mut patset);
}

