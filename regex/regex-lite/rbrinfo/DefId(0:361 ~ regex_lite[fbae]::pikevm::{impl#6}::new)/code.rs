fn new() -> SlotTable {
        SlotTable { table: vec![], slots_for_captures: 0, slots_per_state: 0 }
    }