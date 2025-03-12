// Answer 0

#[test]
fn test_fmt_with_valid_transitions() {
    let transitions = vec![
        Transition { start: 97, end: 122, next: StateID::new_unchecked(2) }, // a to z
        Transition { start: 65, end: 90, next: StateID::new_unchecked(3) },  // A to Z
    ];
    let state = State { transitions };
    let _ = format!("{:?}", state);
}

#[test]
fn test_fmt_with_single_transition() {
    let transitions = vec![
        Transition { start: 48, end: 57, next: StateID::new_unchecked(4) }, // 0 to 9
    ];
    let state = State { transitions };
    let _ = format!("{:?}", state);
}

#[test]
fn test_fmt_with_overlapping_transitions() {
    let transitions = vec![
        Transition { start: 30, end: 60, next: StateID::new_unchecked(5) }, // 30 to 60
        Transition { start: 61, end: 100, next: StateID::new_unchecked(6) }, // 61 to 100
    ];
    let state = State { transitions };
    let _ = format!("{:?}", state);
}

#[test]
fn test_fmt_with_edge_case_transitions() {
    let transitions = vec![
        Transition { start: 1, end: 1, next: StateID::new_unchecked(7) }, // minimum range
        Transition { start: 255, end: 255, next: StateID::new_unchecked(8) }, // maximum range
    ];
    let state = State { transitions };
    let _ = format!("{:?}", state);
}

