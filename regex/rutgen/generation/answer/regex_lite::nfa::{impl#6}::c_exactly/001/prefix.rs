// Answer 0

#[test]
fn test_c_exactly_zero() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::SomeKind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let result = compiler.c_exactly(&hir, 0);
}

#[test]
fn test_c_exactly_one() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::SomeKind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let result = compiler.c_exactly(&hir, 1);
}

#[test]
fn test_c_exactly_max() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::SomeKind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let result = compiler.c_exactly(u32::MAX);
}

#[test]
#[should_panic]
fn test_c_exactly_negative() {
    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("test"));
    let hir = Hir { kind: HirKind::SomeKind, is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let result = compiler.c_exactly(&hir, -1);
}

