// Answer 0

#[test]
fn test_pop_root_valid_case() {
    let mut builder = Builder {
        config: Config::default(),
        ..Default::default()
    };
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            trans: vec![Transition { start: 0, end: 255, next: StateID(0) }],
            last: None,
        }],
    };
    
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.pop_root();
}

#[test]
#[should_panic]
fn test_pop_root_empty_uncompiled() {
    let mut builder = Builder {
        config: Config::default(),
        ..Default::default()
    };
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![], // empty to trigger panic
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.pop_root(); // This should panic because uncompiled is empty.
}

