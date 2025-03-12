// Answer 0

#[test]
fn test_all_absent_valid_length() {
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 10],
        slots_per_state: 5,
        slots_for_captures: 2,
    };
    let result = slot_table.all_absent();
}

#[test]
fn test_all_absent_minimum_captures() {
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 1],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let result = slot_table.all_absent();
}

#[test]
#[should_panic]
fn test_all_absent_exceeding_captures() {
    let mut slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap())); 5],
        slots_per_state: 5,
        slots_for_captures: 10,
    };
    let result = slot_table.all_absent();
}

#[test]
fn test_all_absent_zero_length() {
    let mut slot_table = SlotTable {
        table: Vec::new(),
        slots_per_state: 0,
        slots_for_captures: 0,
    };
    let result = slot_table.all_absent();
}

