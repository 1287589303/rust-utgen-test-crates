// Answer 0

#[test]
fn test_take_to_save_none() {
    let mut state_saver = StateSaver::None;
    let result = state_saver.take_to_save();
}

#[test]
fn test_take_to_save_saved() {
    let lazy_state_id = LazyStateID(42);
    let state = State {
        id: 1,
        is_match: true,
        ntrans: 2,
        input_ranges: &[1, 2, 3],
        next: &[0, 1, 2],
        pattern_ids: &[1, 2, 3],
        accel: &[0],
    };
    let mut state_saver = StateSaver::ToSave { id: lazy_state_id, state };
    let result = state_saver.take_to_save();
}

