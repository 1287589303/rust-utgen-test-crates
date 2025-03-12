// Answer 0

#[test]
fn test_reverse_suffix_new_auto_prefilter_disabled() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(false), &[]),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let hirs: Vec<&Hir> = vec![];
    let _ = ReverseSuffix::new(core, &hirs);
}

#[test]
fn test_reverse_suffix_new_always_anchored_start() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true), &[]),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    // Simulate the anchored start condition
    let _ = ReverseSuffix::new(core, &[]);
}

#[test]
fn test_reverse_suffix_new_no_dfa_or_hybrid() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true), &[]),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: None,
        dfa: None,
    };
    let hirs: Vec<&Hir> = vec![];
    let _ = ReverseSuffix::new(core, &hirs);
}

#[test]
fn test_reverse_suffix_new_prefilter_fast() {
    let prefilter = Prefilter::new(MatchKind::LeftmostFirst, &[b"needle"]).unwrap();
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true), &[]),
        pre: Some(prefilter.clone()),
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let hirs: Vec<&Hir> = vec![];
    let _ = ReverseSuffix::new(core, &hirs);
}

#[test]
fn test_reverse_suffix_new_lcs_none() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true), &[]),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let hirs: Vec<&Hir> = vec![];
    let _ = ReverseSuffix::new(core, &hirs);
}

#[test]
fn test_reverse_suffix_new_lcs_empty() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true), &[]),
        pre: None,
        nfa: NFA::default(),
        nfarev: None,
        pikevm: wrappers::PikeVM::default(),
        backtrack: wrappers::BoundedBacktracker::default(),
        onepass: wrappers::OnePass::default(),
        hybrid: wrappers::Hybrid::default(),
        dfa: wrappers::DFA::default(),
    };
    let hirs: Vec<&Hir> = vec![];
    let _ = ReverseSuffix::new(core, &hirs);
}

