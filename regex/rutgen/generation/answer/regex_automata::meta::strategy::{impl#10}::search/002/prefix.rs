// Answer 0

#[test]
fn test_search_with_non_anchored_input_quadratic_error() {
    let haystack: &[u8] = b"test input with various patterns";
    let span = Span::new(0, haystack.len()); // Valid span covering the entire haystack
    let anchored = Anchored::No;

    let input = Input::new(haystack).span(span).anchored(anchored);
    
    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA::new(Arc::new(Inner::default())),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let strategy = ReverseInner {
        core,
        preinner: Prefilter::default(),
        nfarev: NFA::new(Arc::new(Inner::default())),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let mut cache = Cache::default();
    
    strategy.search(&mut cache, &input);
}

#[test]
fn test_search_with_non_anchored_input_fail_error() {
    let haystack: &[u8] = b"example input with different patterns";
    let span = Span::new(0, haystack.len()); // Valid span covering the whole haystack
    let anchored = Anchored::No;

    let input = Input::new(haystack).span(span).anchored(anchored);
    
    let core = Core {
        info: RegexInfo::default(),
        pre: None,
        nfa: NFA::new(Arc::new(Inner::default())),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };

    let strategy = ReverseInner {
        core,
        preinner: Prefilter::default(),
        nfarev: NFA::new(Arc::new(Inner::default())),
        hybrid: wrappers::ReverseHybrid::default(),
        dfa: wrappers::ReverseDFA::default(),
    };

    let mut cache = Cache::default();
    
    strategy.search(&mut cache, &input);
}

