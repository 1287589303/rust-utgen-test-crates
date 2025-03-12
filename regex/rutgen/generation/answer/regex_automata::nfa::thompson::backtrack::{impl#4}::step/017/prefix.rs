// Answer 0

#[test]
fn test_step_byte_range_return_none() {
    let cache = &mut Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let haystack = b"abcdef";
    let input = Input::new(&haystack).set_range(0..haystack.len());
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = 0;
    let mut slots = vec![None; 1];

    let trans = Transition {
        start: b'a',
        end: b'z',
        next: StateID(SmallIndex::new(1).unwrap()),
    };

    let nfa = NFA(vec![State::ByteRange { trans: trans }.into()]);
    let backtracker = BoundedBacktracker { nfa, config: Config::default() };

    // Insert the visited state first
    cache.visited.insert(sid, at - input.start());

    let result = backtracker.step(cache, &input, sid, at, &mut slots);
}

#[test]
fn test_step_byte_range_return_none_on_no_match() {
    let cache = &mut Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let haystack = b"abcdef";
    let input = Input::new(&haystack).set_range(0..haystack.len());
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = 0;
    let mut slots = vec![None; 1];

    let trans = Transition {
        start: b'x', // This transition will not match 'a'
        end: b'y',
        next: StateID(SmallIndex::new(1).unwrap()),
    };

    let nfa = NFA(vec![State::ByteRange { trans: trans }.into()]);
    let backtracker = BoundedBacktracker { nfa, config: Config::default() };

    // Insert the visited state first
    cache.visited.insert(sid, at - input.start());

    let result = backtracker.step(cache, &input, sid, at, &mut slots);
}

#[test]
fn test_step_byte_range_return_none_if_visited() {
    let cache = &mut Cache {
        stack: Vec::new(),
        visited: Visited {
            bitset: vec![0; 1],
            stride: 1,
        },
    };
    let haystack = b"abcdef";
    let input = Input::new(&haystack).set_range(0..haystack.len());
    let sid = StateID(SmallIndex::new(0).unwrap());
    let at = 0;
    let mut slots = vec![None; 1];

    let trans = Transition {
        start: b'a',
        end: b'z',
        next: StateID(SmallIndex::new(1).unwrap()),
    };

    let nfa = NFA(vec![State::ByteRange { trans: trans }.into()]);
    let backtracker = BoundedBacktracker { nfa, config: Config::default() };

    // Insert the visited state first
    cache.visited.insert(sid, at - input.start());

    // Insert the same state again to cause a revisit
    cache.visited.insert(sid, at - input.start());

    let result = backtracker.step(cache, &input, sid, at, &mut slots);
}

