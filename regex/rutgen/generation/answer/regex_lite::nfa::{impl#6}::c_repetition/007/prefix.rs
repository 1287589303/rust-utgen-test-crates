// Answer 0

#[test]
fn test_c_repetition_zero_or_one() {
    let rep = hir::Repetition {
        min: 0,
        max: Some(1),
        greedy: true,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::SomeKind, // Replace with a concrete HirKind variant
            is_start_anchored: false,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("a*b"));
    compiler.c_repetition(&rep).unwrap();
}

#[test]
fn test_c_repetition_at_least() {
    let rep = hir::Repetition {
        min: 1,
        max: None,
        greedy: false,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::SomeKind, // Replace with a concrete HirKind variant
            is_start_anchored: true,
            is_match_empty: false,
            static_explicit_captures_len: None,
        }),
    };

    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("c+d"));
    compiler.c_repetition(&rep).unwrap();
}

#[test]
fn test_c_repetition_exactly() {
    let rep = hir::Repetition {
        min: 2,
        max: Some(2),
        greedy: true,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::SomeKind, // Replace with a concrete HirKind variant
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: Some(1),
        }),
    };

    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("x{2}"));
    compiler.c_repetition(&rep).unwrap();
}

#[test]
fn test_c_repetition_bounded() {
    let rep = hir::Repetition {
        min: 2,
        max: Some(5),
        greedy: false,
        sub: Box::new(hir::Hir {
            kind: hir::HirKind::SomeKind, // Replace with a concrete HirKind variant
            is_start_anchored: false,
            is_match_empty: true,
            static_explicit_captures_len: Some(1),
        }),
    };

    let config = Config { nest_limit: 10, flags: Flags::default() };
    let compiler = Compiler::new(config, String::from("y{2,5}"));
    compiler.c_repetition(&rep).unwrap();
}

