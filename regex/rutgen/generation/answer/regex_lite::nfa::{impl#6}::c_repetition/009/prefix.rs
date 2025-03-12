// Answer 0

#[test]
fn test_c_repetition_zero_or_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let sub_hir = Hir { kind: HirKind::Simple, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let rep = Repetition { min: 0, max: Some(1), greedy: true, sub: Box::new(sub_hir) };
    
    let _result = compiler.c_repetition(&rep);
}

#[test]
fn test_c_repetition_at_least() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let sub_hir = Hir { kind: HirKind::Simple, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let rep = Repetition { min: 2, max: None, greedy: false, sub: Box::new(sub_hir) };
    
    let _result = compiler.c_repetition(&rep);
}

