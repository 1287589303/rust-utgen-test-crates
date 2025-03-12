// Answer 0

#[test]
fn test_nexts_with_valid_input() {
    let haystack: &[u8] = b"test input";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };

    let state_id = StateID(SmallIndex::new(1));
    let slot_table = SlotTable { 
        table: vec![Some(NonMaxUsize(1)); 2], 
        slots_per_state: 2, 
        slots_for_captures: 2 
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 2];
    let mut stack: Vec<FollowEpsilon> = Vec::new();
    let sparse_set = SparseSet {
        len: 1,
        dense: vec![state_id],
        sparse: vec![state_id],
    };

    let curr = ActiveStates {
        set: sparse_set,
        slot_table,
    };
    let mut next = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };

    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };

    let pike_vm = PikeVM {
        config,
        nfa: NFA(Arc::new(Inner::default())),
    };

    let at = 0;

    let _pid = pike_vm.nexts(
        &mut stack,
        &mut curr,
        &mut next,
        &input,
        at,
        &mut slots,
    );
}

#[test]
fn test_nexts_with_empty_slots() {
    let haystack: &[u8] = b"sample";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };

    let state_id = StateID(SmallIndex::new(2));
    let slot_table = SlotTable {
        table: vec![],
        slots_per_state: 1,
        slots_for_captures: 0,
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![];
    let mut stack: Vec<FollowEpsilon> = Vec::new();
    let sparse_set = SparseSet {
        len: 1,
        dense: vec![state_id],
        sparse: vec![state_id],
    };

    let curr = ActiveStates {
        set: sparse_set,
        slot_table,
    };
    let mut next = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };

    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };

    let pike_vm = PikeVM {
        config,
        nfa: NFA(Arc::new(Inner::default())),
    };

    let at = 0;

    let _pid = pike_vm.nexts(
        &mut stack,
        &mut curr,
        &mut next,
        &input,
        at,
        &mut slots,
    );
}

#[test]
fn test_nexts_with_long_input() {
    let haystack: &[u8] = b"regex-automata testing";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };

    let state_id = StateID(SmallIndex::new(3));
    let slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(2)); 2],
        slots_per_state: 2,
        slots_for_captures: 2,
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 2];
    let mut stack: Vec<FollowEpsilon> = Vec::new();
    let sparse_set = SparseSet {
        len: 1,
        dense: vec![state_id],
        sparse: vec![state_id],
    };

    let curr = ActiveStates {
        set: sparse_set,
        slot_table,
    };
    let mut next = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };

    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };

    let pike_vm = PikeVM {
        config,
        nfa: NFA(Arc::new(Inner::default())),
    };

    let at = 0;

    let _pid = pike_vm.nexts(
        &mut stack,
        &mut curr,
        &mut next,
        &input,
        at,
        &mut slots,
    );
}

#[test]
fn test_nexts_with_boundary_at_index() {
    let haystack: &[u8] = b"boundary test";
    let span = Span::new(0, haystack.len());
    let input = Input {
        haystack,
        span,
        anchored: Anchored::No,
        earliest: false,
    };

    let state_id = StateID(SmallIndex::new(4));
    let slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(3)); 2],
        slots_per_state: 2,
        slots_for_captures: 2,
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 2];
    let mut stack: Vec<FollowEpsilon> = Vec::new();
    let sparse_set = SparseSet {
        len: 1,
        dense: vec![state_id],
        sparse: vec![state_id],
    };

    let curr = ActiveStates {
        set: sparse_set,
        slot_table,
    };
    let mut next = ActiveStates {
        set: SparseSet::new(0),
        slot_table: SlotTable::new(),
    };

    let config = Config {
        match_kind: Some(MatchKind::All),
        ..Default::default()
    };

    let pike_vm = PikeVM {
        config,
        nfa: NFA(Arc::new(Inner::default())),
    };

    let at = 1; // boundary condition with at

    let _pid = pike_vm.nexts(
        &mut stack,
        &mut curr,
        &mut next,
        &input,
        at,
        &mut slots,
    );
}

