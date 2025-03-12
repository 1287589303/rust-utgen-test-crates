// Answer 0

#[test]
fn test_c_with_empty_class() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags::empty() };
    let mut compiler = Compiler::new(config, String::from(".*"));
    let class = Class { ranges: Vec::new() };
    let hir = Hir { kind: HirKind::Class(class), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_with_single_char_in_class() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags::empty() };
    let mut compiler = Compiler::new(config, String::from("a"));
    let class = Class { ranges: vec![ClassRange { start: 'a' as u32, end: 'a' as u32 }] };
    let hir = Hir { kind: HirKind::Class(class), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_with_range_in_class() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags::empty() };
    let mut compiler = Compiler::new(config, String::from("[a-z]"));
    let class = Class { ranges: vec![ClassRange { start: 'a' as u32, end: 'z' as u32 }] };
    let hir = Hir { kind: HirKind::Class(class), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_with_multiple_ranges_in_class() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags::empty() };
    let mut compiler = Compiler::new(config, String::from("[a-zA-Z]"));
    let class = Class { ranges: vec![ClassRange { start: 'a' as u32, end: 'z' as u32 }, ClassRange { start: 'A' as u32, end: 'Z' as u32 }] };
    let hir = Hir { kind: HirKind::Class(class), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _ = compiler.c(&hir);
}

#[test]
fn test_c_with_empty_ranges_in_class() {
    let config = Config { size_limit: None, nest_limit: 10, flags: Flags::empty() };
    let mut compiler = Compiler::new(config, String::from("[^]"));
    let class = Class { ranges: vec![] };
    let hir = Hir { kind: HirKind::Class(class), is_start_anchored: false, is_match_empty: false, static_explicit_captures_len: None };
    let _ = compiler.c(&hir);
}

