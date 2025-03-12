// Answer 0

#[test]
fn test_new_backtrack_true_match_kind_not_leftmost_first() {
    let info = {
        let config = Config::default()
            .backtrack(true)
            .match_kind(MatchKind::All);
        RegexInfo(Arc::new(RegexInfo::new(config, &[])))
    };
    let pre: Option<Prefilter> = None;
    let nfa = NFA(Arc::new(Inner::default()));

    let result = BoundedBacktrackerEngine::new(&info, pre, &nfa);
}

#[test]
fn test_new_backtrack_true_match_kind_leftmost_first() {
    let info = {
        let config = Config::default()
            .backtrack(true)
            .match_kind(MatchKind::LeftmostFirst);
        RegexInfo(Arc::new(RegexInfo::new(config, &[])))
    };
    let pre: Option<Prefilter> = None;
    let nfa = NFA(Arc::new(Inner::default()));

    let result = BoundedBacktrackerEngine::new(&info, pre, &nfa);
}

