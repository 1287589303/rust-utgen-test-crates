// Answer 0

#[test]
fn test_add_with_prefix_len_equal_to_ranges_len() {
    let mut builder = Builder {
        pattern_id: None,
        states: Vec::new(),
        start_pattern: Vec::new(),
        captures: Vec::new(),
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
                trans: Vec::new(),
                last: Some(Utf8LastTransition { start: 0, end: 5 }),
            },
            Utf8Node {
                trans: Vec::new(),
                last: Some(Utf8LastTransition { start: 5, end: 10 }),
            },
        ],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    
    let ranges = vec![
        Utf8Range::new(0, 5),
        Utf8Range::new(5, 10),
    ];

    let _ = compiler.add(&ranges);
}

