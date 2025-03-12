// Answer 0

#[test]
fn test_remap_capture_valid_next() {
    let mut state = State::Capture {
        next: StateID(SmallIndex(1)),
        pattern_id: PatternID(SmallIndex(0)),
        group_index: SmallIndex(0),
        slot: SmallIndex(0),
    };

    let remap = vec![StateID(SmallIndex(0)), StateID(SmallIndex(2))];

    state.remap(&remap);
}

#[test]
fn test_remap_capture_multiple_states() {
    let mut state = State::Capture {
        next: StateID(SmallIndex(0)),
        pattern_id: PatternID(SmallIndex(1)),
        group_index: SmallIndex(0),
        slot: SmallIndex(1),
    };

    let remap = vec![
        StateID(SmallIndex(5)),
        StateID(SmallIndex(1)),
        StateID(SmallIndex(4)),
    ];

    state.remap(&remap);
}

#[test]
fn test_remap_capture_next_to_end() {
    let mut state = State::Capture {
        next: StateID(SmallIndex(3)),
        pattern_id: PatternID(SmallIndex(2)),
        group_index: SmallIndex(1),
        slot: SmallIndex(0),
    };

    let remap = vec![
        StateID(SmallIndex(1)),
        StateID(SmallIndex(4)),
        StateID(SmallIndex(2)),
        StateID(SmallIndex(3)),
    ];

    state.remap(&remap);
}

#[test]
fn test_remap_capture_with_empty_remap() {
    let mut state = State::Capture {
        next: StateID(SmallIndex(0)),
        pattern_id: PatternID(SmallIndex(3)),
        group_index: SmallIndex(0),
        slot: SmallIndex(1),
    };

    let remap: Vec<StateID> = vec![];

    state.remap(&remap);
}

