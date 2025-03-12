// Answer 0

#[test]
fn test_reverse_suffix_new_auto_prefilter_true_anchored_start_true() {
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

    let result = ReverseSuffix::new(core, &hirs);
}

#[test]
fn test_reverse_suffix_new_auto_prefilter_true_anchored_start_true_with_hirs() {
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
    let hirs: Vec<&Hir> = vec![&Hir::from_literal("test")];

    let result = ReverseSuffix::new(core, &hirs);
}

