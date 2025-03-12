// Answer 0

#[test]
fn test_remap_match_state() {
    let mut state = State::Match { pattern_id: PatternID(SmallIndex(0)) };
    let remap: Vec<StateID> = (0..256).map(|i| StateID(SmallIndex(i))).collect();
    state.remap(&remap);
}

#[test]
fn test_remap_match_state_with_empty_remap() {
    let mut state = State::Match { pattern_id: PatternID(SmallIndex(1)) };
    let remap: Vec<StateID> = Vec::new();
    state.remap(&remap);
}

#[test]
fn test_remap_match_state_with_full_remap() {
    let mut state = State::Match { pattern_id: PatternID(SmallIndex(2)) };
    let remap: Vec<StateID> = (0..256).map(|i| StateID(SmallIndex(255 - i))).collect();
    state.remap(&remap);
}

