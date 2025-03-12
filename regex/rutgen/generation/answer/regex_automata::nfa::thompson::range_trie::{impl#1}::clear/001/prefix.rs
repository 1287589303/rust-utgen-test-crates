// Answer 0

#[test]
fn test_clear_no_transitions() {
    let mut state = State { transitions: vec![] };
    state.clear();
}

#[test]
fn test_clear_one_transition() {
    let mut state = State {
        transitions: vec![Transition {
            range: Utf8Range { start: 0, end: 0 },
            next_id: StateID::new_unchecked(2),
        }],
    };
    state.clear();
}

#[test]
fn test_clear_multiple_transitions() {
    let mut state = State {
        transitions: vec![
            Transition {
                range: Utf8Range { start: 0, end: 0 },
                next_id: StateID::new_unchecked(2),
            },
            Transition {
                range: Utf8Range { start: 255, end: 255 },
                next_id: StateID::new_unchecked(3),
            },
        ],
    };
    state.clear();
}

#[test]
fn test_clear_boundary_transitions() {
    let mut state = State {
        transitions: vec![
            Transition {
                range: Utf8Range { start: 0, end: 0 },
                next_id: StateID::new_unchecked(1),
            },
            Transition {
                range: Utf8Range { start: 255, end: 255 },
                next_id: StateID::new_unchecked(4),
            },
            Transition {
                range: Utf8Range { start: 1, end: 254 },
                next_id: StateID::new_unchecked(5),
            },
        ],
    };
    state.clear();
}

