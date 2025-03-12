// Answer 0

#[test]
fn test_c_bounded_ok_case() {
    let config = Config { nest_limit: 5, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a{2,5}"));
    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let min = 2;
    let max = 5;
    let greedy = true;

    // Call the function under test
    let _result = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
#[should_panic]
fn test_c_bounded_invalid_splits() {
    let config = Config { nest_limit: 0, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a{2,5}"));
    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let min = 2;
    let max = 3;
    let greedy = false;

    // Call the function under test
    let _result = compiler.c_bounded(&hir, greedy, min, max);
}

