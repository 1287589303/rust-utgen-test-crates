// Answer 0

#[test]
fn test_top_last_freeze_valid_case() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: true,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
        #[cfg(feature = "syntax")]
        thompson: thompson::Compiler::default(),
    };
    
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node { trans: vec![], last: None },
        ],
    };
    
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    let next = StateID(SmallIndex::from(1));
    
    compiler.top_last_freeze(next);
}

#[test]
#[should_panic(expected = "non-empty nodes")]
fn test_top_last_freeze_empty_nodes() {
    let mut builder = Builder {
        pattern_id: None,
        states: vec![],
        start_pattern: vec![],
        captures: vec![],
        memory_states: 0,
        utf8: true,
        reverse: false,
        look_matcher: LookMatcher::default(),
        size_limit: None,
        config: Config::default(),
        #[cfg(feature = "syntax")]
        thompson: thompson::Compiler::default(),
    };
    
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![],
    };
    
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    let next = StateID(SmallIndex::from(1));
    
    compiler.top_last_freeze(next);
}

