// Answer 0

#[test]
fn test_reverse_suffix_auto_prefilter_true_anchored_start_false_hybrid_false_dfa_false() {
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
    let hirs: &[&Hir] = &[];

    let result = ReverseSuffix::new(core, hirs);
}

#[test]
fn test_reverse_suffix_auto_prefilter_true_anchored_start_false_hybrid_false_dfa_false_empty_hirs() {
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
    let hirs: &[&Hir] = &[];

    let result = ReverseSuffix::new(core, hirs);
}

#[test]
fn test_reverse_suffix_auto_prefilter_true_anchored_start_false_hybrid_false_dfa_false_single_hir() {
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
    let literal_hir = literal::Literal::from_str("test").into();
    let hirs: &[&Hir] = &[&literal_hir];

    let result = ReverseSuffix::new(core, hirs);
}

#[test]
fn test_reverse_suffix_auto_prefilter_true_anchored_start_false_hybrid_false_dfa_false_multiple_hirs() {
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
    let literal_hir_1 = literal::Literal::from_str("foo").into();
    let literal_hir_2 = literal::Literal::from_str("bar").into();
    let hirs: &[&Hir] = &[&literal_hir_1, &literal_hir_2];

    let result = ReverseSuffix::new(core, hirs);
}

