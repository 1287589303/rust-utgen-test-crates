// Answer 0

#[test]
fn test_remap_states_with_valid_transitions() {
    let mut inner = Inner {
        states: vec![
            State { 
                id: StateID(SmallIndex(0)), 
                is_match: false, 
                ntrans: 1, 
                input_ranges: &[], 
                next: &[], 
                pattern_ids: &[], 
                accel: &[] 
            },
        ],
        start_anchored: StateID(SmallIndex(0)),
        start_unanchored: StateID(SmallIndex(0)),
        start_pattern: vec![StateID(SmallIndex(0))],
        ..Default::default()
    };
    let old_to_new = vec![StateID(SmallIndex(1))];
    inner.remap(&old_to_new);
}

#[test]
fn test_remap_states_without_valid_transitions() {
    let mut inner = Inner {
        states: vec![
            State { 
                id: StateID(SmallIndex(1)), 
                is_match: true, 
                ntrans: 0,
                input_ranges: &[], 
                next: &[], 
                pattern_ids: &[], 
                accel: &[] 
            },
        ],
        start_anchored: StateID(SmallIndex(1)),
        start_unanchored: StateID(SmallIndex(1)),
        start_pattern: vec![StateID(SmallIndex(1))],
        ..Default::default()
    };
    let old_to_new = vec![StateID(SmallIndex(2))];
    inner.remap(&old_to_new);
}

#[test]
fn test_remap_with_valid_pattern_ids() {
    let mut inner = Inner {
        states: vec![
            State { 
                id: StateID(SmallIndex(2)), 
                is_match: false, 
                ntrans: 1, 
                input_ranges: &[], 
                next: &[], 
                pattern_ids: &[], 
                accel: &[] 
            },
        ],
        start_anchored: StateID(SmallIndex(2)),
        start_unanchored: StateID(SmallIndex(2)),
        start_pattern: vec![StateID(SmallIndex(0))],
        ..Default::default()
    };
    let old_to_new = vec![StateID(SmallIndex(1))];
    inner.remap(&old_to_new);
}

#[test]
fn test_remap_with_invalid_pattern_ids() {
    let mut inner = Inner {
        states: vec![
            State { 
                id: StateID(SmallIndex(3)), 
                is_match: true, 
                ntrans: 0, 
                input_ranges: &[], 
                next: &[], 
                pattern_ids: &[], 
                accel: &[] 
            },
        ],
        start_anchored: StateID(SmallIndex(3)),
        start_unanchored: StateID(SmallIndex(3)),
        start_pattern: vec![StateID(SmallIndex(1))],
        ..Default::default()
    };
    let old_to_new: Vec<StateID> = vec![StateID(SmallIndex(4))];
    inner.remap(&old_to_new);
}

#[test]
fn test_remap_with_empty_mapping() {
    let mut inner = Inner {
        states: vec![
            State { 
                id: StateID(SmallIndex(5)), 
                is_match: false, 
                ntrans: 0, 
                input_ranges: &[], 
                next: &[], 
                pattern_ids: &[], 
                accel: &[] 
            },
        ],
        start_anchored: StateID(SmallIndex(5)),
        start_unanchored: StateID(SmallIndex(5)),
        start_pattern: vec![StateID(SmallIndex(5))],
        ..Default::default()
    };
    let old_to_new: Vec<StateID> = vec![];
    inner.remap(&old_to_new);
}

