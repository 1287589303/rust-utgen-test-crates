// Answer 0

#[test]
fn test_is_match_nofail_onepass() {
    let info = RegexInfo(Arc::new(RegexInfoI {}));
    let nfa = NFA(Arc::new(Inner {}));
    let prefilter = Some(Prefilter {
        pre: Arc::new(DummyPrefilter {}),
        is_fast: true,
        max_needle_len: 64,
    });

    let core = Core {
        info,
        pre: prefilter.clone(),
        nfa,
        nfarev: None,
        pikevm: wrappers::PikeVM {},
        backtrack: wrappers::BoundedBacktracker {},
        onepass: wrappers::OnePass {},
        hybrid: wrappers::Hybrid {},
        dfa: wrappers::DFA {},
    };

    let input = Input {
        haystack: &[1],
        span: Span::new(0, 1),
        anchored: Anchored::True,
        earliest: true,
    };

    let mut cache = Cache {
        capmatches: Captures {},
        pikevm: wrappers::PikeVMCache {},
        backtrack: wrappers::BoundedBacktrackerCache {},
        onepass: wrappers::OnePassCache {},
        hybrid: wrappers::HybridCache {},
        revhybrid: wrappers::ReverseHybridCache {},
    };

    core.is_match_nofail(&mut cache, &input);
}

#[test]
fn test_is_match_nofail_backtrack() {
    let info = RegexInfo(Arc::new(RegexInfoI {}));
    let nfa = NFA(Arc::new(Inner {}));
    let prefilter = Some(Prefilter {
        pre: Arc::new(DummyPrefilter {}),
        is_fast: false,
        max_needle_len: 256,
    });

    let core = Core {
        info,
        pre: prefilter.clone(),
        nfa,
        nfarev: None,
        pikevm: wrappers::PikeVM {},
        backtrack: wrappers::BoundedBacktracker {},
        onepass: wrappers::OnePass {},
        hybrid: wrappers::Hybrid {},
        dfa: wrappers::DFA {},
    };

    let input = Input {
        haystack: &[0; 128],
        span: Span::new(0, 128),
        anchored: Anchored::False,
        earliest: false,
    };

    let mut cache = Cache {
        capmatches: Captures {},
        pikevm: wrappers::PikeVMCache {},
        backtrack: wrappers::BoundedBacktrackerCache {},
        onepass: wrappers::OnePassCache {},
        hybrid: wrappers::HybridCache {},
        revhybrid: wrappers::ReverseHybridCache {},
    };

    core.is_match_nofail(&mut cache, &input);
}

