// Answer 0

#[test]
fn test_step_sparse_state_none_return() {
    let mut cache = Cache {
        stack: vec![],
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };

    let inputs = vec![b"a", b"b", b"c"];
    let input = Input::new(&inputs).set_range(0..3);

    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = 1;

    let sparse_transition = SparseTransitions {
        transitions: Box::new([]),
    };
    let nfa = NFA(Arc::new(Inner {
        states: vec![State::Sparse(sparse_transition)],
    }));

    let backtracker = BoundedBacktracker {
        config: Config::default(),
        nfa: nfa.clone(),
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 3];

    cache.visited.insert(sid, at - input.start());
    let result = backtracker.step(&mut cache, &input, sid, at, &mut slots);

    // Testing for expected None return
    assert!(result.is_none());
}

#[test]
fn test_step_sparse_state_not_matched_return() {
    let mut cache = Cache {
        stack: vec![],
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };

    let inputs = vec![b"x", b"y", b"z"];
    let input = Input::new(&inputs).set_range(0..3);
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = 1;

    let sparse_transition = SparseTransitions {
        transitions: Box::new([]), // No valid transitions
    };
    let nfa = NFA(Arc::new(Inner {
        states: vec![State::Sparse(sparse_transition)],
    }));

    let backtracker = BoundedBacktracker {
        config: Config::default(),
        nfa: nfa.clone(),
    };

    let mut slots: Vec<Option<NonMaxUsize>> = vec![None; 3];

    cache.visited.insert(sid, at - input.start());
    let result = backtracker.step(&mut cache, &input, sid, at, &mut slots);

    // Testing for expected None return due to sparse transitions
    assert!(result.is_none());
}

