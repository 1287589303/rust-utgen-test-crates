// Answer 0

#[test]
fn test_add_match_state() {
    let mut nfa_inner = Inner::default();
    let match_state = State::Match { pattern_id: PatternID::new(0).unwrap() };
    let _id = nfa_inner.add(match_state);
}

#[test]
fn test_add_multiple_match_states() {
    let mut nfa_inner = Inner::default();
    for pattern_id in 0..10 {
        let match_state = State::Match { pattern_id: PatternID::new(pattern_id).unwrap() };
        let _id = nfa_inner.add(match_state);
    }
}

#[test]
#[should_panic]
fn test_add_exceed_capacity() {
    let mut nfa_inner = Inner::default();
    for pattern_id in 0..1000 { // Assuming capacity limit is reached
        let match_state = State::Match { pattern_id: PatternID::new(pattern_id).unwrap() };
        let _id = nfa_inner.add(match_state);
    }
}

