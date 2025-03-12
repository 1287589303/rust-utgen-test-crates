// Answer 0

#[test]
fn test_next_with_transitions_and_dead_state() {
    let transition_dead = Transition { start: 0, end: 0, next: DEAD };
    let transition_active = Transition { start: 1, end: 1, next: StateID::new(1) };

    let it = vec![transition_dead, transition_active]; // Two transitions: one DEAD, one active
    let iter = it.iter().enumerate();
    
    let mut sparse_transition_iter = SparseTransitionIter {
        it: iter,
        cur: Some((0, 0, transition_dead)),
    };

    let result = sparse_transition_iter.next(); // Call the function under test
}

#[test]
fn test_next_with_contiguous_transitions_and_dead_state() {
    let transition_dead = Transition { start: 2, end: 2, next: DEAD };
    let transition_active = Transition { start: 2, end: 2, next: StateID::new(1) };

    let it = vec![transition_active, transition_dead]; // Both transitions are contiguous with one DEAD
    let iter = it.iter().enumerate();

    let mut sparse_transition_iter = SparseTransitionIter {
        it: iter,
        cur: Some((2, 2, transition_active)),
    };

    let result = sparse_transition_iter.next(); // Call the function under test
}

#[test]
fn test_next_with_non_dead_transitions() {
    let transition_dead = Transition { start: 3, end: 3, next: DEAD };
    let transition_active = Transition { start: 4, end: 4, next: StateID::new(2) };

    let it = vec![transition_active, transition_dead]; // A sequence ending with DEAD
    let iter = it.iter().enumerate();

    let mut sparse_transition_iter = SparseTransitionIter {
        it: iter,
        cur: Some((4, 4, transition_active)),
    };

    let result = sparse_transition_iter.next(); // Call the function under test
}

