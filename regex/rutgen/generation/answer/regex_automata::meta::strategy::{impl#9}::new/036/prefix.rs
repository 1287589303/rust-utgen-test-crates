// Answer 0

#[test]
fn test_reverse_inner_new_precondition_fail() {
    let core = Core {
        info: RegexInfo::new(Config::new()
            .auto_prefilter(true)
            .which_captures(WhichCaptures::None)
            .match_kind(MatchKind::LeftmostFirst)
            .nfa_size_limit(Some(1024)),
            &[]),
        pre: None,
        hybrid: None,
        dfa: Some(DFA::new(&RegexInfo::new(Config::default(), &[]), None, &NFA::default(), &NFA::default())),
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
}

#[test]
fn test_reverse_inner_new_extract_none() {
    let core = Core {
        info: RegexInfo::new(Config::new()
            .auto_prefilter(true)
            .which_captures(WhichCaptures::None)
            .match_kind(MatchKind::LeftmostFirst)
            .nfa_size_limit(Some(10))),
        pre: None,
        hybrid: None,
        dfa: Some(DFA::new(&RegexInfo::new(Config::default(), &[]), None, &NFA::default(), &NFA::default())),
    };
    let hirs: &[&Hir] = &[];

    let result = ReverseInner::new(core, hirs);
}

