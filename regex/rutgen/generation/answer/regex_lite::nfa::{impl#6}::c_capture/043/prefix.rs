// Answer 0

#[test]
fn test_c_capture_case_1() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("test pattern"));
    
    let hir = Hir { kind: HirKind::Empty, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    
    let result = compiler.c_capture(0, Some("test"), &hir);
}

#[test]
fn test_c_capture_case_2() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let mut compiler = Compiler::new(config, String::from("test pattern"));
    
    let hir = Hir { kind: HirKind::Empty, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    
    compiler.nfa.borrow_mut().cap_index_to_name.push(Some(Arc::from("first")));
    
    let result = compiler.c_capture(0, Some("test"), &hir);
}

