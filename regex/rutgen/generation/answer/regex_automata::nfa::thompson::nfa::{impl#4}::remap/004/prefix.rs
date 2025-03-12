// Answer 0

#[test]
fn test_remap_binary_union_valid() {
    let mut state = State::BinaryUnion {
        alt1: StateID(SmallIndex(1)),
        alt2: StateID(SmallIndex(2)),
    };
    let remap = [
        StateID(SmallIndex(0)),
        StateID(SmallIndex(3)),
        StateID(SmallIndex(4)),
        StateID(SmallIndex(5)),
    ];
    state.remap(&remap);
}

#[test]
fn test_remap_binary_union_boundary_high() {
    let mut state = State::BinaryUnion {
        alt1: StateID(SmallIndex(254)),
        alt2: StateID(SmallIndex(255)),
    };
    let remap = [
        StateID(SmallIndex(0)),
        StateID(SmallIndex(1)),
        StateID(SmallIndex(2)),
        // ... (up to 255)
        StateID(SmallIndex(254)),
        StateID(SmallIndex(255)),
    ];
    state.remap(&remap);
}

#[test]
fn test_remap_binary_union_boundary_low() {
    let mut state = State::BinaryUnion {
        alt1: StateID(SmallIndex(0)),
        alt2: StateID(SmallIndex(1)),
    };
    let remap = [
        StateID(SmallIndex(0)),
        StateID(SmallIndex(2)),
        StateID(SmallIndex(3)),
        // ... (up to 255)
    ];
    state.remap(&remap);
}

#[test]
#[should_panic]
fn test_remap_binary_union_invalid() {
    let mut state = State::BinaryUnion {
        alt1: StateID(SmallIndex(5)),
        alt2: StateID(SmallIndex(10)),
    };
    let remap = [
        StateID(SmallIndex(0)),
        StateID(SmallIndex(1)),
        StateID(SmallIndex(2)),
        // ... (only creating a short array)
    ];
    state.remap(&remap);
}

