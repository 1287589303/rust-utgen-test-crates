// Answer 0

#[test]
fn test_epsilon_closure_case_1() {
    let stack = &mut vec![
        FollowEpsilon::RestoreCapture {
            slot: SmallIndex(0),
            offset: Some(NonMaxUsize::new(1).unwrap()),
        },
    ];
    
    let curr_slots = &mut [Some(NonMaxUsize::new(0).unwrap())];
    let next = &mut ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    
    let input = Input {
        haystack: b"abc".as_ref(),
        span: Span::new(0, 3),
        anchored: Anchored::No,
        earliest: false,
    };
    
    let at = 0;
    let sid = StateID(SmallIndex(1));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(stack, curr_slots, next, &input, at, sid);
}

#[test]
fn test_epsilon_closure_case_2() {
    let stack = &mut vec![
        FollowEpsilon::RestoreCapture {
            slot: SmallIndex(1),
            offset: Some(NonMaxUsize::new(2).unwrap()),
        },
    ];
    
    let curr_slots = &mut [Some(NonMaxUsize::new(3).unwrap()), None];
    let next = &mut ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    
    let input = Input {
        haystack: b"xyz".as_ref(),
        span: Span::new(0, 3),
        anchored: Anchored::No,
        earliest: false,
    };
    
    let at = 1;
    let sid = StateID(SmallIndex(2));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(stack, curr_slots, next, &input, at, sid);
}

#[test]
fn test_epsilon_closure_case_3() {
    let stack = &mut vec![
        FollowEpsilon::RestoreCapture {
            slot: SmallIndex(2),
            offset: None,
        },
    ];
    
    let curr_slots = &mut [None, Some(NonMaxUsize::new(4).unwrap()), Some(NonMaxUsize::new(5).unwrap())];
    let next = &mut ActiveStates {
        set: SparseSet::default(),
        slot_table: SlotTable::default(),
    };
    
    let input = Input {
        haystack: b"hello".as_ref(),
        span: Span::new(0, 5),
        anchored: Anchored::No,
        earliest: false,
    };
    
    let at = 2;
    let sid = StateID(SmallIndex(3));

    let pike_vm = PikeVM {
        config: Config::default(),
        nfa: NFA::default(),
    };

    pike_vm.epsilon_closure(stack, curr_slots, next, &input, at, sid);
}

