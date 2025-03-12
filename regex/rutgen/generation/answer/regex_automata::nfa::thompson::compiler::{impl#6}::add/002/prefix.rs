// Answer 0

#[test]
fn test_add_with_matching_prefix() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition { start: 0, end: 1 }),
            },
            Utf8Node {
                trans: vec![],
                last: None,
            },
        ],
    };
    let target = StateID(0);
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    
    let ranges = vec![
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 1, end: 2 },
    ];
    
    let _ = compiler.add(&ranges).unwrap();
}

#[test]
fn test_add_with_non_matching_suffix() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition { start: 0, end: 1 }),
            },
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition { start: 2, end: 3 }),
            },
        ],
    };
    let target = StateID(0);
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    
    let ranges = vec![
        Utf8Range { start: 0, end: 1 },
        Utf8Range { start: 3, end: 4 },
        Utf8Range { start: 5, end: 6 },
    ];
    
    let _ = compiler.add(&ranges).unwrap();
}

