// Answer 0

#[test]
fn test_step_sparse_state_end_index() {
    let cache = Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1], // Initialize a small bitset for testing
            stride: 1,
        },
    };

    let haystack: &[u8] = b"test haystack";
    let span = Span::new(0, haystack.len());
    let input = Input::new(haystack).span(span);
    
    let sid = StateID(SmallIndex::new(0).unwrap()); // Valid StateID
    let at = input.end(); // Set at to the end of the input

    let transitions = SparseTransitions {
        transitions: vec![
            Transition { start: b't', end: b't', next: StateID(SmallIndex::new(1).unwrap()) },
            // Add more transitions if necessary
        ].into_boxed_slice(),
    };

    let nfa = NFA { 
        // Setup your NFA structure with at least a Sparse state at the given sid
    };

    let backtracker = BoundedBacktracker { 
        config: Config::default(), 
        nfa,
    };
    
    let result = backtracker.step(&mut cache, &input, sid, at, &mut vec![None]);

    // expected to be None
}

#[test]
fn test_step_sparse_state_no_transition_at_end() {
    let cache = Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1], 
            stride: 1,
        },
    };

    let haystack: &[u8] = b"";
    let span = Span::new(0, 0);
    let input = Input::new(haystack).span(span);
    
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = input.end(); // at equals input.end()

    let transitions = SparseTransitions {
        transitions: vec![
            Transition { start: b'a', end: b'z', next: StateID(SmallIndex::new(2).unwrap()) },
        ].into_boxed_slice(),
    };

    let nfa = NFA { 
        // Ensure the Sparse state returns None as there are no transitions at the end 
    };

    let backtracker = BoundedBacktracker { 
        config: Config::default(), 
        nfa,
    };

    let result = backtracker.step(&mut cache, &input, sid, at, &mut vec![None]);

    // expected to be None
}

