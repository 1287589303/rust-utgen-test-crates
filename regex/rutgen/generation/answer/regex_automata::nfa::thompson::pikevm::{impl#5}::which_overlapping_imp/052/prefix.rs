// Answer 0

#[test]
fn test_which_overlapping_imp_input_done_false_haystack_max() {
    let haystack = vec![0u8; core::usize::MAX];
    let input = Input::new(&haystack).set_span(0..haystack.len()).set_earliest(false);
    let mut cache = Cache::new(&PikeVM {
        config: Config::new(),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut patset = PatternSet::new(10);
    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_input_done_false_haystack_empty() {
    let haystack = vec![];
    let input = Input::new(&haystack).set_span(0..0).set_earliest(false);
    let mut cache = Cache::new(&PikeVM {
        config: Config::new(),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut patset = PatternSet::new(10);
    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_input_start_end_within_bounds() {
    let haystack = vec![1u8, 2u8, 3u8];
    let input = Input::new(&haystack).set_span(0..3).set_earliest(true);
    let mut cache = Cache::new(&PikeVM {
        config: Config::new(),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut patset = PatternSet::new(10);
    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_multiple_ranges() {
    let haystack = vec![b'a', b'b', b'a', b'c', b'd'];
    let input = Input::new(&haystack).set_span(0..5).set_earliest(false);
    let mut cache = Cache::new(&PikeVM {
        config: Config::new(),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut patset = PatternSet::new(10);
    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pike_vm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

