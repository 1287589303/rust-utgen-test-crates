// Answer 0

#[test]
fn test_next_boundary_case_empty() {
    let it: &[StateID] = &[];
    let mut iter = StateTransitionIter {
        len: 0,
        it: it.iter().enumerate(),
    };
    let result = iter.next();
}

#[test]
fn test_next_boundary_case_single() {
    let state_id = StateID(Default::default());
    let it: &[StateID] = &[state_id];
    let mut iter = StateTransitionIter {
        len: 1,
        it: it.iter().enumerate(),
    };
    let result = iter.next();
}

#[test]
fn test_next_boundary_case_two() {
    let state_id_0 = StateID(Default::default());
    let state_id_1 = StateID(Default::default());
    let it: &[StateID] = &[state_id_0, state_id_1];
    let mut iter = StateTransitionIter {
        len: 2,
        it: it.iter().enumerate(),
    };
    let result_0 = iter.next();
    let result_1 = iter.next();
}

#[test]
fn test_next_boundary_case_multiple() {
    let state_ids: Vec<StateID> = (0..5).map(|_| StateID(Default::default())).collect();
    let mut iter = StateTransitionIter {
        len: 5,
        it: state_ids.iter().enumerate(),
    };
    for _ in 0..5 {
        let result = iter.next();
    }
}

