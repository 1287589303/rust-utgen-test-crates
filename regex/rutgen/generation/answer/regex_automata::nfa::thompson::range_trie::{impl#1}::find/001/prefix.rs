// Answer 0

#[test]
fn test_find_empty_transitions() {
    let state = State {
        transitions: vec![],
    };
    let range = Utf8Range { start: 0, end: 0 };
    state.find(range);
}

#[test]
fn test_find_single_transition() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 1, end: 2 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 0, end: 0 };
    state.find(range);
}

#[test]
fn test_find_transition_before() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 1, end: 2 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 0, end: 0 };
    state.find(range);
}

#[test]
fn test_find_transition_after() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 1, end: 2 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 3, end: 3 };
    state.find(range);
}

#[test]
fn test_find_transition_overlap() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 1, end: 2 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 1, end: 1 };
    state.find(range);
}

#[test]
fn test_find_transition_overlap_full() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 1, end: 3 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 2, end: 2 };
    state.find(range);
}

#[test]
fn test_find_transition_boundaries() {
    let state = State {
        transitions: vec![
            Transition { range: Utf8Range { start: 0, end: 0 }, next_id: ROOT },
            Transition { range: Utf8Range { start: 2, end: 2 }, next_id: ROOT },
        ],
    };
    let range = Utf8Range { start: 0, end: 0 };
    state.find(range);
}

#[test]
fn test_find_transition_high_range() {
    let state = State {
        transitions: vec![Transition { range: Utf8Range { start: 250, end: 255 }, next_id: ROOT }],
    };
    let range = Utf8Range { start: 0, end: 255 };
    state.find(range);
}

#[test]
#[should_panic]
fn test_find_invalid_range() {
    let state = State {
        transitions: vec![],
    };
    let range = Utf8Range { start: 10, end: 1 }; // Invalid since start > end
    state.find(range);
}

