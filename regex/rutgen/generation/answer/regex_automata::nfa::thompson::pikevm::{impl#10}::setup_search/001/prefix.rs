// Answer 0

#[test]
fn test_setup_search_zero_captures() {
    let mut slot_table = SlotTable {
        table: vec![None; 10],
        slots_per_state: 10,
        slots_for_captures: 0,
    };
    slot_table.setup_search(0);
}

#[test]
fn test_setup_search_equal_to_slots_per_state() {
    let mut slot_table = SlotTable {
        table: vec![None; 10],
        slots_per_state: 10,
        slots_for_captures: 0,
    };
    slot_table.setup_search(10);
}

#[test]
fn test_setup_search_maximum_value_below_slots_per_state() {
    let mut slot_table = SlotTable {
        table: vec![None; 10],
        slots_per_state: 10,
        slots_for_captures: 0,
    };
    slot_table.setup_search(9);
}

#[test]
#[should_panic]
fn test_setup_search_exceeds_slots_per_state() {
    let mut slot_table = SlotTable {
        table: vec![None; 10],
        slots_per_state: 10,
        slots_for_captures: 0,
    };
    slot_table.setup_search(11); // should panic
}

