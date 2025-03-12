// Answer 0

#[test]
fn test_nexts_valid_case() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(0)))];
    let curr_set = SparseSet::new(1);
    let curr_slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap()))],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let curr = ActiveStates {
        set: curr_set,
        slot_table: curr_slot_table,
    };
    let next = ActiveStates {
        set: SparseSet::new(1),
        slot_table: SlotTable::new(),
    };
    let input = Input {
        haystack: b"test".as_slice(),
        span: Span::new(0, 4),
        anchored: Anchored::No,
        earliest: false,
    };
    let at = 0;
    let mut slots = vec![None; 1];

    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA::default(),
    };

    let pid = pike_vm.nexts(&mut stack, &mut curr, &mut next, &input, at, &mut slots);
}

#[test]
fn test_nexts_partial_match_case() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(1)))];
    let curr_set = SparseSet::new(1);
    let curr_slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(2).unwrap()))],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let curr = ActiveStates {
        set: curr_set,
        slot_table: curr_slot_table,
    };
    let next = ActiveStates {
        set: SparseSet::new(1),
        slot_table: SlotTable::new(),
    };
    let input = Input {
        haystack: b"sample".as_slice(),
        span: Span::new(0, 6),
        anchored: Anchored::No,
        earliest: false,
    };
    let at = 1;
    let mut slots = vec![None; 1];

    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::LeftmostFirst),
        nfa: NFA::default(),
    };

    let pid = pike_vm.nexts(&mut stack, &mut curr, &mut next, &input, at, &mut slots);
}

#[test]
fn test_nexts_no_continue_case() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(2)))];
    let curr_set = SparseSet::new(1);
    let curr_slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(3).unwrap()))],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let curr = ActiveStates {
        set: curr_set,
        slot_table: curr_slot_table,
    };
    let next = ActiveStates {
        set: SparseSet::new(1),
        slot_table: SlotTable::new(),
    };
    let input = Input {
        haystack: b"example".as_slice(),
        span: Span::new(0, 7),
        anchored: Anchored::No,
        earliest: false,
    };
    let at = 2;
    let mut slots = vec![None; 1];

    let pike_vm = PikeVM {
        config: Config::new().match_kind(MatchKind::All), // This should allow for continuation
        nfa: NFA::default(),
    };

    let pid = pike_vm.nexts(&mut stack, &mut curr, &mut next, &input, at, &mut slots);
}

