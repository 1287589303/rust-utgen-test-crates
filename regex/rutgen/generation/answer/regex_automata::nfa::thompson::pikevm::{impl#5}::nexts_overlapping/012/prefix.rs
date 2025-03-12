// Answer 0

#[test]
fn test_nexts_overlapping_case_1() {
    let mut stack = Vec::new();
    let mut curr = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable {
            table: vec![None; 10],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let mut next = ActiveStates {
        set: SparseSet::new(10),
        slot_table: SlotTable {
            table: vec![None; 10],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let input = Input::new(&b"test string"[..]).anchored(Anchored::No).earliest(true);
    let at = 0;
    
    let mut patset = PatternSet::new(5);
    let nfa = NFA::always_match();
    let pike_vm = PikeVM {
        config: Config::default().match_kind(MatchKind::LeftmostFirst),
        nfa,
    };
    
    curr.set.insert(StateID(SmallIndex::new(1)));
    let _ = pike_vm.nexts_overlapping(&mut stack, &mut curr, &mut next, &input, at, &mut patset);
}

#[test]
fn test_nexts_overlapping_case_2() {
    let mut stack = Vec::new();
    let mut curr = ActiveStates {
        set: SparseSet::new(5),
        slot_table: SlotTable {
            table: vec![Some(NonMaxUsize::new(0).unwrap()); 5],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let mut next = ActiveStates {
        set: SparseSet::new(5),
        slot_table: SlotTable {
            table: vec![Some(NonMaxUsize::new(1).unwrap()); 5],
            slots_per_state: 2,
            slots_for_captures: 2,
        },
    };
    let input = Input::new(&b"another test"[..]).anchored(Anchored::No).earliest(false);
    let at = 2;
    
    let mut patset = PatternSet::new(5);
    let nfa = NFA::never_match();
    let pike_vm = PikeVM {
        config: Config::default().match_kind(MatchKind::All),
        nfa,
    };
    
    curr.set.insert(StateID(SmallIndex::new(2)));
    let _ = pike_vm.nexts_overlapping(&mut stack, &mut curr, &mut next, &input, at, &mut patset);
}

