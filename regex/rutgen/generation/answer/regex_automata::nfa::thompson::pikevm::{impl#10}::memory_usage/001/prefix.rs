// Answer 0

#[test]
fn test_memory_usage_empty_table() {
    let slot_table = SlotTable {
        table: vec![],
        slots_per_state: 1,
        slots_for_captures: 0,
    };
    let _usage = slot_table.memory_usage();
}

#[test]
fn test_memory_usage_single_slot() {
    let slot_table = SlotTable {
        table: vec![Some(NonMaxUsize(NonZeroUsize::new(1).unwrap()))],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let _usage = slot_table.memory_usage();
}

#[test]
fn test_memory_usage_ten_slots() {
    let slot_table = SlotTable {
        table: (1..=10).map(|i| Some(NonMaxUsize(NonZeroUsize::new(i).unwrap()))).collect(),
        slots_per_state: 1,
        slots_for_captures: 10,
    };
    let _usage = slot_table.memory_usage();
}

#[test]
fn test_memory_usage_hundred_slots() {
    let slot_table = SlotTable {
        table: (1..=100).map(|i| Some(NonMaxUsize(NonZeroUsize::new(i).unwrap()))).collect(),
        slots_per_state: 1,
        slots_for_captures: 100,
    };
    let _usage = slot_table.memory_usage();
}

