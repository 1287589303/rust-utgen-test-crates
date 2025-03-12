// Answer 0

#[test]
fn test_c_with_capture() {
    let config = Config { size_limit: None };
    let pattern = "a";
    let compiler = Compiler::new(config, pattern.to_string());

    let capture_sub_hir = Hir::char('b');
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(capture_sub_hir),
    };
    let hir = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _result = compiler.c(&hir);
}

#[test]
fn test_c_with_capture_with_name() {
    let config = Config { size_limit: None };
    let pattern = "a";
    let compiler = Compiler::new(config, pattern.to_string());

    let capture_sub_hir = Hir::char('c');
    let capture_name = Some(Box::from("my_capture"));
    let capture = Capture {
        index: 1,
        name: capture_name,
        sub: Box::new(capture_sub_hir),
    };
    let hir = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };

    let _result = compiler.c(&hir);
}

#[test]
fn test_c_with_capture_non_empty_hir() {
    let config = Config { size_limit: None };
    let pattern = "a";
    let compiler = Compiler::new(config, pattern.to_string());

    let inner_capture_sub_hir = Hir::char('d');
    let outer_capture_sub_hir = Hir::concat(vec![
        Hir::char('e'),
        Hir::capture(Capture {
            index: 2,
            name: None,
            sub: Box::new(inner_capture_sub_hir),
        }),
    ]);
    let capture = Capture {
        index: 2,
        name: Some(Box::from("outer_capture")),
        sub: Box::new(outer_capture_sub_hir),
    };
    let hir = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(1),
    };

    let _result = compiler.c(&hir);
}

