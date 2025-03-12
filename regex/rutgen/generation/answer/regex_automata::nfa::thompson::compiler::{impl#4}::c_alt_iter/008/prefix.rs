// Answer 0

#[test]
fn test_c_alt_iter_success_first_failure_on_second() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let first_thompson_ref = ThompsonRef {
        start: StateID(1),
        end: StateID(2),
    };

    let second_thompson_ref = ThompsonRef {
        start: StateID(3),
        end: StateID(4),
    };

    let valid_thompson_refs = vec![
        Ok(first_thompson_ref),
        Ok(second_thompson_ref),
    ];

    let mut it = valid_thompson_refs.into_iter();

    let _ = compiler.add_union().unwrap();
    let _ = compiler.add_empty().unwrap();

    let _ = compiler.patch(StateID(1), StateID(5)).unwrap(); // patch first.start
    let _ = compiler.patch(StateID(2), StateID(6)).unwrap(); // patch first.end

    // Simulate the second patch operation to return an Err
    let result = compiler.patch(StateID(3), StateID(7)); // patch second.start
    assert!(result.is_err()); // Expect the second patch to fail
}

#[test]
fn test_c_alt_iter_success_with_multiple_valid_refs() {
    let compiler = Compiler {
        parser: ParserBuilder::default(),
        config: Config::default(),
        builder: RefCell::new(Builder {
            config: Config::default(),
        }),
        utf8_state: RefCell::new(Utf8State {
            compiled: Utf8BoundedMap::default(),
            uncompiled: Vec::new(),
        }),
        trie_state: RefCell::new(RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        }),
        utf8_suffix: RefCell::new(Utf8SuffixMap {
            version: 0,
            capacity: 0,
            map: vec![],
        }),
    };

    let first_thompson_ref = ThompsonRef {
        start: StateID(1),
        end: StateID(2),
    };

    let second_thompson_ref = ThompsonRef {
        start: StateID(3),
        end: StateID(4),
    };

    let third_thompson_ref = ThompsonRef {
        start: StateID(5),
        end: StateID(6),
    };

    let valid_thompson_refs = vec![
        Ok(first_thompson_ref),
        Ok(second_thompson_ref),
        Ok(third_thompson_ref),
    ];

    let mut it = valid_thompson_refs.into_iter();

    let union_id = compiler.add_union().unwrap();
    let end_id = compiler.add_empty().unwrap();

    let _ = compiler.patch(union_id, StateID(1)).unwrap(); // patch first.start
    let _ = compiler.patch(StateID(2), end_id).unwrap(); // patch first.end
    let _ = compiler.patch(union_id, StateID(3)).unwrap(); // patch second.start
    let _ = compiler.patch(StateID(4), end_id).unwrap(); // patch second.end
    
    // Simulate a valid patch operation for the third element
    let _ = compiler.patch(union_id, StateID(5)).unwrap(); // patch third.start
    let _ = compiler.patch(StateID(6), end_id).unwrap(); // patch third.end
}

