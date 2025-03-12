// Answer 0

#[test]
fn test_c_bounded_empty_hir_greedy() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(""));

    let hir = Hir { kind: HirKind::Empty, is_start_anchored: false, is_match_empty: true, static_explicit_captures_len: None };
    let greedy = true;
    let min = 0;
    let max = 0;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_char_hir_greedy() {
    let config = Config { nest_limit: 1, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a"));

    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = true;
    let min = 0;
    let max = 0;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_empty_hir_not_greedy() {
    let config = Config { nest_limit: 2, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from(""));

    let hir = Hir { kind: HirKind::Empty, is_start_anchored: false, is_match_empty: true, static_explicit_captures_len: None };
    let greedy = false;
    let min = 0;
    let max = 0;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_char_hir_not_greedy() {
    let config = Config { nest_limit: 2, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a"));

    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = false;
    let min = 0;
    let max = 0;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_char_hir_min_max_equal_one() {
    let config = Config { nest_limit: 3, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a"));

    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = true;
    let min = 1;
    let max = 1;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_char_hir_min_max_equal_two() {
    let config = Config { nest_limit: 4, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a"));

    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let greedy = false;
    let min = 2;
    let max = 2;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

