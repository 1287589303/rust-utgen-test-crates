// Answer 0

#[test]
fn test_finish_success_valid_utf8_ranges() {
    let mut builder = Builder {
        pattern_id: Some(PatternID(1)),
        states: Vec::new(),
        start_pattern: vec![StateID(1)],
        captures: Vec::new(),
        memory_states: 0,
        utf8: true,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
    };
    
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node { trans: vec![], last: None }],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.add(&[Utf8Range::new(0, 127)]).unwrap();
    compiler.add(&[Utf8Range::new(128, 255)]).unwrap();
    compiler.finish().unwrap();
}

#[test]
fn test_finish_success_valid_target() {
    let mut builder = Builder {
        pattern_id: Some(PatternID(1)),
        states: Vec::new(),
        start_pattern: vec![StateID(2)],
        captures: Vec::new(),
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
    };
    
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node { trans: vec![], last: None }],
    };

    let target_state = StateID(3);
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.target = target_state;
    compiler.add(&[Utf8Range::new(0, 127)]).unwrap();
    compiler.finish().unwrap();
}

#[test]
fn test_finish_success_multiple_nodes() {
    let mut builder = Builder {
        pattern_id: Some(PatternID(1)),
        states: Vec::new(),
        start_pattern: vec![StateID(4)],
        captures: Vec::new(),
        memory_states: 0,
        utf8: true,
        reverse: true,
        look_matcher: LookMatcher::default(),
        size_limit: None,
    };
    
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node { trans: vec![], last: None },
            Utf8Node { trans: vec![], last: None }
        ],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.add(&[Utf8Range::new(0, 127)]).unwrap();
    compiler.add(&[Utf8Range::new(128, 255)]).unwrap();
    let thompson_ref = compiler.finish().unwrap();
    assert!(thompson_ref.start.0.0 > 0); // Ensure the state ID is valid
}

