// Answer 0

#[test]
fn test_memory_usage_empty_active_states() {
    let sparse_set = SparseSet::new(0);
    let slot_table = SlotTable::new();
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_sparse_set_capacity_zero() {
    let sparse_set = SparseSet::new(0);
    let slot_table = SlotTable {
        table: vec![None; 0],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_sparse_set_capacity_small() {
    let mut sparse_set = SparseSet::new(5);
    for i in 0..5 {
        sparse_set.insert(i as StateID);
    }
    let slot_table = SlotTable {
        table: vec![None; 2 * 5],
        slots_per_state: 2,
        slots_for_captures: 2,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_sparse_set_capacity_large() {
    let mut sparse_set = SparseSet::new(1000);
    for i in 0..1000 {
        sparse_set.insert(i as StateID);
    }
    let slot_table = SlotTable {
        table: vec![None; 1000],
        slots_per_state: 5,
        slots_for_captures: 5,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_slot_table_with_single_slot() {
    let sparse_set = SparseSet::new(10);
    let slot_table = SlotTable {
        table: vec![None; 10],
        slots_per_state: 1,
        slots_for_captures: 1,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_slot_table_with_multiple_slots() {
    let sparse_set = SparseSet::new(20);
    let slot_table = SlotTable {
        table: vec![None; 40],
        slots_per_state: 2,
        slots_for_captures: 2,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

#[test]
fn test_memory_usage_large_states_various_slots() {
    let mut sparse_set = SparseSet::new(1000);
    for i in 0..1000 {
        sparse_set.insert(i as StateID);
    }
    let slot_table = SlotTable {
        table: vec![None; 5000],
        slots_per_state: 5,
        slots_for_captures: 5,
    };
    let active_states = ActiveStates { set: sparse_set, slot_table };
    let _ = active_states.memory_usage();
}

