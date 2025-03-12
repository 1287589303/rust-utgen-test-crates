// Answer 0

#[test]
fn test_add_suffix_single_element() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            trans: Vec::new(),
            last: None,
        }],
    };
    let ranges = vec![Utf8Range { start: 5, end: 5 }];
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.add_suffix(&ranges);
}

#[test]
#[should_panic(expected = "empty nodes")]
fn test_add_suffix_empty_uncompiled() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: Vec::new(),
    };
    let ranges = vec![Utf8Range { start: 5, end: 5 }];
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.add_suffix(&ranges);
}

#[test]
fn test_add_suffix_multiple_elements() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            trans: Vec::new(),
            last: None,
        }],
    };
    let ranges = vec![
        Utf8Range { start: 3, end: 3 },
        Utf8Range { start: 4, end: 4 },
    ];
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();
    compiler.add_suffix(&ranges);
}

