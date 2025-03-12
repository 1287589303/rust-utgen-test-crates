// Answer 0

#[test]
fn test_add_with_valid_ranges_and_should_compile() {
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
    };

    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node {
                trans: vec![],
                last: None,
            },
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition {
                    start: 1,
                    end: 2,
                }),
            }
        ],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    
    let ranges: Vec<Utf8Range> = vec![
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
    ];

    compiler.add(&ranges).unwrap();
}

#[test]
fn test_add_with_invalid_ranges_and_should_compile() {
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
    };

    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition {
                    start: 3,
                    end: 4,
                }),
            },
            Utf8Node {
                trans: vec![],
                last: None,
            }
        ],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();

    let ranges: Vec<Utf8Range> = vec![
        Utf8Range { start: 1, end: 2 },
        Utf8Range { start: 5, end: 6 },
    ];

    compiler.add(&ranges).unwrap();
}

