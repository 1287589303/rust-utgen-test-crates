// Answer 0

#[test]
fn test_set_with_invalid_to_state() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 10], // Minimum size for testing
        classes,
        stride2: 1,
    };

    let from_state = StateID(0); // Assuming this is valid
    let unit = alphabet::Unit(UnitKind::U8(1)); // Assuming this is a valid unit

    // Intentionally creating an invalid to_state
    let to_state = StateID(10); // Assuming 10 is out of bounds

    // This should panic due to invalid 'to' state
    table.set(from_state, unit, to_state);
}

#[test]
fn test_set_with_edge_case_invalid_to_state() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 1], // Edge case with minimum size
        classes,
        stride2: 1,
    };

    let from_state = StateID(0); // Valid state
    let unit = alphabet::Unit(UnitKind::U8(0)); // Valid unit

    // Intentionally creating an invalid to_state
    let to_state = StateID(1); // Assuming 1 is out of bounds for this small table

    // This should panic due to invalid 'to' state
    table.set(from_state, unit, to_state);
}

