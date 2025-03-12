// Answer 0

#[test]
fn test_sparse_transition_iter_next_case_1() {
    let trans = Transition { start: 0, end: 0, next: StateID(0) }; // DEAD state
    let transitions = vec![trans]; // Contains one transition which is DEAD
    let mut iter = SparseTransitionIter {
        it: transitions.iter().enumerate(),
        cur: Some((0, 0, trans)),
    };
    let result = iter.next();
}

#[test]
fn test_sparse_transition_iter_next_case_2() {
    let trans = Transition { start: 1, end: 1, next: StateID(0) }; // DEAD state
    let transitions = vec![trans, trans]; // Contains transitions where prev_trans equals trans
    let mut iter = SparseTransitionIter {
        it: transitions.iter().enumerate(),
        cur: Some((1, 1, trans)),
    };
    let result = iter.next();
}

#[test]
fn test_sparse_transition_iter_next_case_3() {
    let trans = Transition { start: 2, end: 2, next: StateID(0) }; // DEAD state
    let transitions = vec![trans, trans, trans]; // All elements are the same
    let mut iter = SparseTransitionIter {
        it: transitions.iter().enumerate(),
        cur: Some((2, 2, trans)),
    };
    let result = iter.next();
}

