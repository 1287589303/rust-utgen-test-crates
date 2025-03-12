// Answer 0

#[test]
fn test_for_state_valid_case() {
    let slots_per_state = 2;
    let slots_for_captures = 1;
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 6],
        slots_per_state,
        slots_for_captures,
    };
    let state_id = StateID(SmallIndex::new(1));
    let result = slot_table.for_state(state_id);
}

#[test]
fn test_for_state_boundary_case_min() {
    let slots_per_state = 3;
    let slots_for_captures = 2;
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 9],
        slots_per_state,
        slots_for_captures,
    };
    let state_id = StateID(SmallIndex::new(0));
    let result = slot_table.for_state(state_id);
}

#[test]
fn test_for_state_boundary_case_max() {
    let slots_per_state = 4;
    let slots_for_captures = 3;
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 16],
        slots_per_state,
        slots_for_captures,
    };
    let state_id = StateID(SmallIndex::new(3));
    let result = slot_table.for_state(state_id);
}

#[test]
fn test_for_state_invalid_case() {
    let slots_per_state = 2;
    let slots_for_captures = 1;
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 6],
        slots_per_state,
        slots_for_captures,
    };
    let state_id = StateID(SmallIndex::new(2)); // Out of range
    let result = slot_table.for_state(state_id);
}

