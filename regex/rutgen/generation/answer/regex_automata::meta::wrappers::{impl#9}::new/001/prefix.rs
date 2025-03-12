// Answer 0

#[test]
fn test_new_hybrid_with_none_prefilter() {
    let info = RegexInfo(Arc::new(RegexInfoI::default()));
    let nfa = NFA(Arc::new(Inner::default()));
    let nfarev = NFA(Arc::new(Inner::default()));
    let hybrid = Hybrid::new(&info, None, &nfa, &nfarev);
}

#[test]
fn test_new_hybrid_with_fast_prefilter() {
    let info = RegexInfo(Arc::new(RegexInfoI::default()));
    let prefilter = Prefilter {
        is_fast: true,
        max_needle_len: 512,
        pre: Arc::new(MockPrefilter {})
    };
    let nfa = NFA(Arc::new(Inner::default()));
    let nfarev = NFA(Arc::new(Inner::default()));
    let hybrid = Hybrid::new(&info, Some(prefilter), &nfa, &nfarev);
}

#[test]
fn test_new_hybrid_with_slow_prefilter() {
    let info = RegexInfo(Arc::new(RegexInfoI::default()));
    let prefilter = Prefilter {
        is_fast: false,
        max_needle_len: 1024,
        pre: Arc::new(MockPrefilter {})
    };
    let nfa = NFA(Arc::new(Inner::default()));
    let nfarev = NFA(Arc::new(Inner::default()));
    let hybrid = Hybrid::new(&info, Some(prefilter), &nfa, &nfarev);
}

