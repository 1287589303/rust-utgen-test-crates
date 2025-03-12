// Answer 0

#[test]
fn test_add_suffix_non_empty_ranges_last_is_some() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            trans: vec![],
            last: None,
        }],
    };
    
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();

    let ranges = vec![
        Utf8Range { start: 65, end: 90 }, // A-Z
    ];

    state.uncompiled.push(Utf8Node {
        trans: vec![],
        last: Some(Utf8LastTransition { start: 0, end: 0 }),
    });

    compiler.add_suffix(&ranges);
}

#[test]
fn test_add_suffix_non_empty_ranges_with_multiple_elements() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![
            Utf8Node {
                trans: vec![],
                last: None,
            },
            Utf8Node {
                trans: vec![],
                last: Some(Utf8LastTransition { start: 0, end: 0 }),
            },
        ],
    };

    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();

    let ranges = vec![
        Utf8Range { start: 10, end: 20 },
        Utf8Range { start: 30, end: 40 },
    ];

    compiler.add_suffix(&ranges);
}

