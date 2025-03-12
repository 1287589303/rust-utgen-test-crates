// Answer 0

#[test]
fn test_add_suffix_non_empty_ranges() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node { trans: vec![], last: None }],
    };
    let mut compiler = Utf8Compiler {
        builder: &mut builder,
        state: &mut state,
        target: StateID(0),
    };
    
    let ranges = vec![
        Utf8Range { start: 0x61, end: 0x61 }, // 'a'
        Utf8Range { start: 0x62, end: 0x62 }, // 'b'
    ];
    
    compiler.add_suffix(&ranges);
}

#[test]
fn test_add_suffix_single_range() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node { trans: vec![], last: None }],
    };
    let mut compiler = Utf8Compiler {
        builder: &mut builder,
        state: &mut state,
        target: StateID(0),
    };
    
    let ranges = vec![
        Utf8Range { start: 0x61, end: 0x61 }, // 'a'
        Utf8Range { start: 0x62, end: 0x62 }, // 'b'
        Utf8Range { start: 0x63, end: 0x63 }, // 'c'
    ];

    compiler.add_suffix(&ranges);
}

