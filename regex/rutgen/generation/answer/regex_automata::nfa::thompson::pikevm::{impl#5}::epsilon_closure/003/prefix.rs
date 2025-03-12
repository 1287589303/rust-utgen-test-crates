// Answer 0

#[test]
fn test_epsilon_closure_valid_case() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(0)))];
    let mut curr_slots = vec![Some(NonMaxUsize::new(0).unwrap())];
    let mut next = ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    let input = Input {
        haystack: &[1, 2, 3],
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let at = 0;
    let sid = StateID(SmallIndex(1));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, &input, at, sid);
}

#[test]
fn test_epsilon_closure_with_multiple_slots() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(2)))];
    let mut curr_slots = vec![
        Some(NonMaxUsize::new(1).unwrap()),
        Some(NonMaxUsize::new(2).unwrap()),
    ];
    let mut next = ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    let input = Input {
        haystack: &[4, 5, 6],
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: true,
    };
    let at = 1;
    let sid = StateID(SmallIndex(3));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, &input, at, sid);
}

#[test]
fn test_epsilon_closure_boundary_condition() {
    let mut stack = vec![FollowEpsilon::Explore(StateID(SmallIndex(4)))];
    let mut curr_slots = vec![None];
    let mut next = ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    let input = Input {
        haystack: &[7],
        span: Span::default(),
        anchored: Anchored::default(),
        earliest: false,
    };
    let at = 0;
    let sid = StateID(SmallIndex(5));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(&mut stack, &mut curr_slots, &mut next, &input, at, sid);
}

