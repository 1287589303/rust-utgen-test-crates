// Answer 0

#[test]
fn test_reverse_inner_new_with_auto_prefilter_true_leftmost_first_false_anchored_start_true() {
    let core = Core {
        info: RegexInfo::new(
            Config::new()
                .auto_prefilter(true)
                .match_kind(MatchKind::LeftmostFirst)
                .line_terminator(b'\n')
                .build(),
            &[],
        ),
        pre: Some(Prefilter {
            is_fast: true,
            max_needle_len: 0,
            pre: Arc::new(()),
        }),
        hybrid: None,
        dfa: None,
        nfa: NFA(Arc::new(Inner)),
    };
    
    let hirs = vec![];
    
    let result = ReverseInner::new(core, &hirs);
}

#[test]
fn test_reverse_inner_new_with_auto_prefilter_true_leftmost_first_false_anchored_start_false() {
    let core = Core {
        info: RegexInfo::new(
            Config::new()
                .auto_prefilter(true)
                .match_kind(MatchKind::LeftmostFirst)
                .line_terminator(b'\n')
                .build(),
            &[],
        ),
        pre: Some(Prefilter {
            is_fast: true,
            max_needle_len: 0,
            pre: Arc::new(()),
        }),
        hybrid: None,
        dfa: None,
        nfa: NFA(Arc::new(Inner)),
    };

    let hirs = vec![];

    let result = ReverseInner::new(core, &hirs);
}

