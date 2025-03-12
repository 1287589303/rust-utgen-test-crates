// Answer 0

#[test]
fn test_step_valid_match() {
    let pattern = "abc";
    let nfa = NFA::new(pattern).unwrap();
    let sid = nfa.start_pattern(PatternID(SmallIndex::new(0).unwrap())).unwrap();
    let input_haystack = b"abc";
    let input = Input::new(&input_haystack[..]).anchored(Anchored::No);
    let mut cache = Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let mut slots = vec![None; 1];

    let result = BoundedBacktracker { config: Config::default(), nfa }.step(&mut cache, &input, sid, 0, &mut slots);
}

#[test]
fn test_step_with_different_pattern() {
    let pattern = "xyz";
    let nfa = NFA::new(pattern).unwrap();
    let sid = nfa.start_pattern(PatternID(SmallIndex::new(0).unwrap())).unwrap();
    let input_haystack = b"xyz";
    let input = Input::new(&input_haystack[..]).anchored(Anchored::No);
    let mut cache = Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let mut slots = vec![None; 1];

    let result = BoundedBacktracker { config: Config::default(), nfa }.step(&mut cache, &input, sid, 0, &mut slots);
}

#[test]
fn test_step_with_non_empty_input() {
    let pattern = "123";
    let nfa = NFA::new(pattern).unwrap();
    let sid = nfa.start_pattern(PatternID(SmallIndex::new(0).unwrap())).unwrap();
    let input_haystack = b"12345";
    let input = Input::new(&input_haystack[..]).anchored(Anchored::No);
    let mut cache = Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let mut slots = vec![None; 1];

    let result = BoundedBacktracker { config: Config::default(), nfa }.step(&mut cache, &input, sid, 0, &mut slots);
}

