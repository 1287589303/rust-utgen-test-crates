// Answer 0

#[test]
fn test_reverse_inner_new_auto_prefilter_false() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true).which_captures(WhichCaptures::None).match_kind(MatchKind::LeftmostFirst), &[]),
        pre: None,
        nfa: NFA(Arc::new(Inner)),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: None,
        dfa: None,
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
    let _ = result.unwrap_err();
}

#[test]
fn test_reverse_inner_new_match_kind_false() {
    let core = Core {
        info: RegexInfo::new(Config::new().auto_prefilter(true).which_captures(WhichCaptures::None).match_kind(MatchKind::All), &[]),
        pre: None,
        nfa: NFA(Arc::new(Inner)),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: None,
        dfa: None,
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
    let _ = result.unwrap_err();
}

#[test]
fn test_reverse_inner_new_anchored_start_false() {
    let core_info = RegexInfo::new(Config::new().auto_prefilter(true).which_captures(WhichCaptures::None).match_kind(MatchKind::LeftmostFirst), &[]);
    let core = Core {
        info: core_info,
        pre: None,
        nfa: NFA(Arc::new(Inner)),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: None,
        dfa: None,
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
    let _ = result.unwrap_err();
}

#[test]
fn test_reverse_inner_new_no_hybrid() {
    let core_info = RegexInfo::new(Config::new().auto_prefilter(true).which_captures(WhichCaptures::None).match_kind(MatchKind::LeftmostFirst), &[]);
    let core = Core {
        info: core_info,
        pre: None,
        nfa: NFA(Arc::new(Inner)),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: None,
        dfa: None,
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
    let _ = result.unwrap_err();
}

#[test]
fn test_reverse_inner_new_no_dfa() {
    let core_info = RegexInfo::new(Config::new().auto_prefilter(true).which_captures(WhichCaptures::None).match_kind(MatchKind::LeftmostFirst), &[]);
    let core = Core {
        info: core_info,
        pre: None,
        nfa: NFA(Arc::new(Inner)),
        nfarev: None,
        pikevm: wrappers::PikeVM::new(),
        backtrack: wrappers::BoundedBacktracker::new(),
        onepass: wrappers::OnePass::new(),
        hybrid: None,
        dfa: None,
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
    let _ = result.unwrap_err();
}

