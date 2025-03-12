// Answer 0

#[test]
fn test_nexts_overlapping_case_one() {
    let mut stack = Vec::new();
    let mut curr = ActiveStates {
        set: SparseSet::new(4),
        slot_table: SlotTable {
            table: vec![None; 4],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let mut next = ActiveStates {
        set: SparseSet::new(4),
        slot_table: SlotTable {
            table: vec![None; 4],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let input = Input::new("test input");
    let at = 0;
    let mut patset = PatternSet::new(4);
    let nfa = NFA::never_match();

    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa,
    };

    pike_vm.nexts_overlapping(&mut stack, &mut curr, &mut next, &input, at, &mut patset);
}

#[test]
fn test_nexts_overlapping_case_two() {
    let mut stack = Vec::new();
    let mut curr = ActiveStates {
        set: SparseSet::new(4),
        slot_table: SlotTable {
            table: vec![Some(0), Some(1), None, None],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let mut next = ActiveStates {
        set: SparseSet::new(4),
        slot_table: SlotTable {
            table: vec![None; 4],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let input = Input::new("another test");
    let at = 5;
    let mut patset = PatternSet::new(4);
    let nfa = NFA::always_match();

    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All),
        nfa,
    };

    pike_vm.nexts_overlapping(&mut stack, &mut curr, &mut next, &input, at, &mut patset);
}

