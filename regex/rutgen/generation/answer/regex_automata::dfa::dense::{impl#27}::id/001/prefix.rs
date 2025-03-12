// Answer 0

#[test]
fn test_state_id_default() {
    let state = State {
        id: StateID(Default::default()),
        stride2: 0,
        transitions: &[],
    };
    let _ = state.id();
}

#[test]
fn test_state_id_min_value() {
    let state = State {
        id: StateID(SmallIndex::new(0).unwrap()),
        stride2: 0,
        transitions: &[],
    };
    let _ = state.id();
}

#[test]
fn test_state_id_max_value() {
    let state = State {
        id: StateID(SmallIndex::new(u32::MAX as usize).unwrap()),
        stride2: 0,
        transitions: &[],
    };
    let _ = state.id();
}

#[test]
fn test_state_id_mid_value() {
    let state = State {
        id: StateID(SmallIndex::new((u32::MAX as usize) / 2).unwrap()),
        stride2: 0,
        transitions: &[],
    };
    let _ = state.id();
}

