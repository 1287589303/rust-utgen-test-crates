// Answer 0

#[test]
fn test_is_accelerated_with_none() {
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI)),
        pre: None,
        nfa: NFA,
        nfarev: None,
        pikevm: wrappers::PikeVM,
        backtrack: wrappers::BoundedBacktracker,
        onepass: wrappers::OnePass,
        hybrid: wrappers::Hybrid,
        dfa: wrappers::DFA,
    };
    core.is_accelerated();
}

#[test]
fn test_is_accelerated_with_prefilter_fast() {
    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilterI {}),
        is_fast: true,
        max_needle_len: 256,
    };
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI)),
        pre: Some(prefilter),
        nfa: NFA,
        nfarev: None,
        pikevm: wrappers::PikeVM,
        backtrack: wrappers::BoundedBacktracker,
        onepass: wrappers::OnePass,
        hybrid: wrappers::Hybrid,
        dfa: wrappers::DFA,
    };
    core.is_accelerated();
}

#[test]
fn test_is_accelerated_with_prefilter_not_fast() {
    let prefilter = Prefilter {
        pre: Arc::new(MyPrefilterI {}),
        is_fast: false,
        max_needle_len: 256,
    };
    let core = Core {
        info: RegexInfo(Arc::new(RegexInfoI)),
        pre: Some(prefilter),
        nfa: NFA,
        nfarev: None,
        pikevm: wrappers::PikeVM,
        backtrack: wrappers::BoundedBacktracker,
        onepass: wrappers::OnePass,
        hybrid: wrappers::Hybrid,
        dfa: wrappers::DFA,
    };
    core.is_accelerated();
}

