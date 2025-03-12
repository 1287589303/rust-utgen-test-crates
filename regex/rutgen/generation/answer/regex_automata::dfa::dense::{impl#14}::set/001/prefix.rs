// Answer 0

#[test]
fn test_set_transition_valid_inputs() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 256], // Initialized with size 256
        classes,
        stride2: 1,
    };
    let from = StateID(SmallIndex::from(1));
    let unit = alphabet::Unit::U8(0);
    let to = StateID(SmallIndex::from(2));
    table.set(from, unit, to);
}

#[test]
fn test_set_transition_boundary_high_from() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 256], // Initialized with size 256
        classes,
        stride2: 1,
    };
    let from = StateID(SmallIndex::from(255));
    let unit = alphabet::Unit::U8(0);
    let to = StateID(SmallIndex::from(254));
    table.set(from, unit, to);
}

#[test]
fn test_set_transition_boundary_low_to() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 256], // Initialized with size 256
        classes,
        stride2: 1,
    };
    let from = StateID(SmallIndex::from(1));
    let unit = alphabet::Unit::U8(0);
    let to = StateID(SmallIndex::from(1));
    table.set(from, unit, to);
}

#[test]
#[should_panic]
fn test_set_transition_invalid_from() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 256], // Initialized with size 256
        classes,
        stride2: 1,
    };
    let from = StateID(SmallIndex::from(256)); // Invalid
    let unit = alphabet::Unit::U8(0);
    let to = StateID(SmallIndex::from(1));
    table.set(from, unit, to);
}

#[test]
#[should_panic]
fn test_set_transition_invalid_to() {
    let mut classes = ByteClasses::singletons();
    let mut table = TransitionTable {
        table: vec![0; 256], // Initialized with size 256
        classes,
        stride2: 1,
    };
    let from = StateID(SmallIndex::from(1));
    let unit = alphabet::Unit::U8(0);
    let to = StateID(SmallIndex::from(256)); // Invalid
    table.set(from, unit, to);
}

