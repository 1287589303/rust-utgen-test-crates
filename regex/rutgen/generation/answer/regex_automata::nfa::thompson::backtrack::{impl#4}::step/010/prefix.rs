// Answer 0

#[test]
fn test_step_dense_case_when_at_is_end() {
    let mut cache = Cache {
        stack: vec![],
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        }
    };
    let input = Input::new(&b"example"[..]).set_range(0..7);
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = input.end(); 
    let mut slots = vec![None; 1];

    let dense_transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::new(1).unwrap()); 256]),
    };
    
    let nfa = NFA(Arc::new(Inner {
        states: vec![State::Dense(dense_transitions)],
        ..Default::default()
    }));
    
    let backtracker = BoundedBacktracker {
        config: Config::default(),
        nfa,
    };

    let result = backtracker.step(&mut cache, &input, sid, at, &mut slots);
}

#[test]
fn test_step_dense_case_at_equals_end() {
    let mut cache = Cache {
        stack: vec![],
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        }
    };
    let input = Input::new(&b"match"[..]).set_range(0..5);
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = input.end();  
    let mut slots = vec![None; 1];

    let dense_transitions = DenseTransitions {
        transitions: Box::new([StateID(SmallIndex::new(1).unwrap()); 256]),
    };
    
    let nfa = NFA(Arc::new(Inner {
        states: vec![State::Dense(dense_transitions)],
        ..Default::default()
    }));
    
    let backtracker = BoundedBacktracker {
        config: Config::default(),
        nfa,
    };

    let result = backtracker.step(&mut cache, &input, sid, at, &mut slots);
}

