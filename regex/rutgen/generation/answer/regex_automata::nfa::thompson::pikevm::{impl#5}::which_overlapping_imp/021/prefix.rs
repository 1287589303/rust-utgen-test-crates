// Answer 0

#[test]
fn test_which_overlapping_imp_case_1() {
    let haystack = b"abcdefg";
    let input = Input::new(&haystack).span(0..6).anchored(Anchored::No);
    let mut patset = PatternSet::new(5);
    let mut cache = Cache::new(&PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut curr = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::default(),
    };
    patset.insert(PatternID(0));

    let pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    };
    
    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case_2() {
    let haystack = b"aaaabaaa";
    let input = Input::new(&haystack).span(0..7).anchored(Anchored::No);
    let mut patset = PatternSet::new(5);
    let mut cache = Cache::new(&PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut curr = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::default(),
    };
    patset.insert(PatternID(0));

    let pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

#[test]
fn test_which_overlapping_imp_case_3() {
    let haystack = b"ababab";
    let input = Input::new(&haystack).span(0..5).anchored(Anchored::No);
    let mut patset = PatternSet::new(5);
    let mut cache = Cache::new(&PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    });
    let mut curr = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::default(),
    };
    patset.insert(PatternID(0));

    let pikevm = PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA(Arc::new(Inner::default())),
    };

    pikevm.which_overlapping_imp(&mut cache, &input, &mut patset);
}

