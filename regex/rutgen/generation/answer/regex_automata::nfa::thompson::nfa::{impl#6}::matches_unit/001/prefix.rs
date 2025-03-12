// Answer 0

#[test]
fn test_matches_unit_valid_byte() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 0, end: 50, next: StateID(SmallIndex(1)) },
            Transition { start: 51, end: 100, next: StateID(SmallIndex(2)) },
        ]),
    };
    
    let unit = Unit::u8(25);
    let result = transitions.matches_unit(unit);
}

#[test]
fn test_matches_unit_another_valid_byte() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 0, end: 100, next: StateID(SmallIndex(1)) },
            Transition { start: 101, end: 200, next: StateID(SmallIndex(2)) },
        ]),
    };
    
    let unit = Unit::u8(150);
    let result = transitions.matches_unit(unit);
}

#[test]
fn test_matches_unit_boundary_start() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 0, end: 10, next: StateID(SmallIndex(1)) },
        ]),
    };
    
    let unit = Unit::u8(0);
    let result = transitions.matches_unit(unit);
}

#[test]
fn test_matches_unit_boundary_end() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 245, end: 255, next: StateID(SmallIndex(1)) },
        ]),
    };
    
    let unit = Unit::u8(255);
    let result = transitions.matches_unit(unit);
}

#[test]
fn test_matches_unit_out_of_bounds() {
    let transitions = SparseTransitions {
        transitions: Box::new([
            Transition { start: 10, end: 20, next: StateID(SmallIndex(1)) },
        ]),
    };
    
    let unit = Unit::u8(30);
    let result = transitions.matches_unit(unit);
}

