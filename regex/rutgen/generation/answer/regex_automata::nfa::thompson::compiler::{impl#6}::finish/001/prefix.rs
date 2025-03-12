// Answer 0

#[test]
fn test_finish_empty_uncompiled() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
    };
    let initial_state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut initial_state).unwrap();
    let result = compiler.finish();
}

#[test]
fn test_finish_single_uncompiled() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
    };
    let initial_state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            last: None,
            trans: vec![],
        }],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut initial_state).unwrap();
    let result = compiler.finish();
}

#[test]
fn test_finish_multiple_uncompiled() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
    };
    let initial_state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node { last: None, trans: vec![] },
            Utf8Node { last: None, trans: vec![] },
        ],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut initial_state).unwrap();
    let result = compiler.finish();
}

#[test]
fn test_finish_valid_target() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
    };
    let initial_state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node { last: None, trans: vec![] },
        ],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut initial_state).unwrap();
    compiler.target = StateID(1);
    let result = compiler.finish();
}

#[test]
fn test_finish_compiled_state_id() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: false,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
    };
    let compiled_map = Utf8BoundedMap::new();
    let initial_state = Utf8State {
        compiled: compiled_map,
        uncompiled: vec![Utf8Node { last: None, trans: vec![] }],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut initial_state).unwrap();
    let result = compiler.finish();
}

