// Answer 0

#[test]
fn test_which_overlapping_matches_dfa_success() {
    let input = Input {
        haystack: &[b'a'; 256],
        span: Span::new(0, 256),
        anchored: Anchored::True,
        earliest: true,
    };
    let mut patset = PatternSet {
        len: 1,
        which: alloc::boxed::Box::new([true; 10]),
    };
    let mut cache = Cache {
        capmatches: Captures::new(),
        pikevm: wrappers::PikeVMCache(None),
        backtrack: wrappers::BoundedBacktrackerCache(None),
        onepass: wrappers::OnePassCache(None),
        hybrid: wrappers::HybridCache(None),
        revhybrid: wrappers::ReverseHybridCache(None),
    };
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI {})),
        pre: None,
        nfa: NFA(Arc::new(Inner {})),
        nfarev: None,
        pikevm: wrappers::PikeVM::none(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::none(),
        hybrid: wrappers::Hybrid::none(),
        dfa: DFA::new(&core.info, core.pre, &core.nfa, &core.nfarev),
    };
    
    core.which_overlapping_matches(&mut cache, &input, &mut patset);
}

