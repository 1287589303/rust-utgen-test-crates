// Answer 0

#[test]
fn test_c_bounded_greedy() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::Char('a'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };

    let min = 1;
    let max = 3;
    let greedy = true;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_non_greedy() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::Char('b'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };

    let min = 2;
    let max = 5;
    let greedy = false;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

#[test]
fn test_c_bounded_boundary() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::Char('c'), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };

    let min = 5;
    let max = 6;
    let greedy = true;

    let _ = compiler.c_bounded(&hir, greedy, min, max);
}

