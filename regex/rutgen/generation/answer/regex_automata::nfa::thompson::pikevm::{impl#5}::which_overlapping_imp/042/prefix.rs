// Answer 0

#[test]
fn test_which_overlapping_imp_case_1() {
    let haystack: &[u8] = b"samwise sam";
    let input = Input::new(&haystack)
        .set_span(0..haystack.len())
        .set_earliest(true);

    let cache = Cache::new(&PikeVM { config: Config::new(), nfa: NFA(Arc::new(Inner::default())) });

    let mut pattern_set = PatternSet::new(10);
    pattern_set.insert(PatternID(0));

    let mut pikevm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    let mut active_states = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::default(),
    };
    active_states.set.insert(StateID(SmallIndex(0)));

    pikevm.which_overlapping_imp(&mut cache, &input, &mut pattern_set);
}

#[test]
fn test_which_overlapping_imp_case_2() {
    let haystack: &[u8] = b"test test test";
    let input = Input::new(&haystack)
        .set_span(0..haystack.len())
        .set_earliest(true);

    let cache = Cache::new(&PikeVM { config: Config::new(), nfa: NFA(Arc::new(Inner::default())) });

    let mut pattern_set = PatternSet::new(10);
    pattern_set.insert(PatternID(1));

    let mut pikevm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa: NFA(Arc::new(Inner::default())),
    };

    let mut active_states = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable::default(),
    };
    active_states.set.insert(StateID(SmallIndex(1)));

    pikevm.which_overlapping_imp(&mut cache, &input, &mut pattern_set);
}

