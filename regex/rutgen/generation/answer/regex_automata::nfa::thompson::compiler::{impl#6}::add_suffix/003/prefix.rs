// Answer 0

#[test]
fn test_add_suffix_single_range() {
    let mut builder = Builder::default();
    let mut state = Utf8State {
        compiled: Utf8BoundedMap::default(),
        uncompiled: vec![Utf8Node {
            trans: vec![],
            last: None,
        }],
    };
    let mut compiler = Utf8Compiler::new(&mut builder, &mut state).unwrap();

    let ranges = vec![Utf8Range {
        start: 0x61, // 'a'
        end: 0x61,   // 'a'
    }];
    
    compiler.add_suffix(&ranges);
}

#[test]
fn test_add_suffix_multiple_ranges() {
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
        Utf8Range {
            start: 0x62, // 'b'
            end: 0x62,   // 'b'
        },
        Utf8Range {
            start: 0x63, // 'c'
            end: 0x63,   // 'c'
        },
    ];

    compiler.add_suffix(&ranges);
}

