// Answer 0

#[test]
fn test_sparse_transition_iter_next() {
    let transition_a = Transition { start: 0, end: 0, next: StateID(1) };
    let transition_b = Transition { start: 1, end: 1, next: StateID(2) };
    let transitions = vec![transition_a, transition_b];
    
    let mut iter = SparseTransitionIter {
        it: transitions.iter().enumerate(),
        cur: Some((0, 0, transition_a)),
    };

    let result = iter.next();
    // result should be Some((0, 0, transition_a))
}

#[test]
fn test_sparse_transition_iter_next_second_transition() {
    let transition_a = Transition { start: 0, end: 0, next: StateID(1) };
    let transition_b = Transition { start: 1, end: 1, next: StateID(3) };
    let transitions = vec![transition_a, transition_b];
    
    let mut iter = SparseTransitionIter {
        it: transitions.iter().enumerate(),
        cur: Some((0, 0, transition_b)),
    };

    let result = iter.next();
    // result should be Some((0, 1, transition_a))
}

