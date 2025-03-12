// Answer 0

#[test]
fn test_c_capture_with_valid_conditions() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut nfa = NFA {
        pattern: String::from("a"),
        states: Vec::new(),
        start: 0,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
        cap_name_to_index: CaptureNameMap::new(),
        cap_index_to_name: Vec::new(),
        memory_extra: 0,
    };
    
    let mut compiler = Compiler { config, nfa: RefCell::new(nfa) };
    let index = 0; // existing_groups_len is also 0
    let name = Some("groupName");
    
    let hir = Hir {
        kind: HirKind::Char('a'),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    // Manually set existing_groups_len to 0
    compiler.nfa.borrow_mut().cap_index_to_name.push(None);
    
    // Call the function under test
    let result = compiler.c_capture(index, name, &hir);

    // The result is expected to return Ok(ThompsonRef)
    let _ = result.unwrap();
}

