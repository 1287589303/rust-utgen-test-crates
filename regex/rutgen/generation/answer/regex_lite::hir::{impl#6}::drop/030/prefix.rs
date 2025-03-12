// Answer 0

#[test]
fn test_drop_with_empty_capture() {
    let empty_hir = Hir::empty();
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(empty_hir),
    };
    let hir_with_capture = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let _ = hir_with_capture; // Test drop on this value
}

#[test]
fn test_drop_with_non_empty_capture() {
    let non_empty_hir = Hir::empty(); // Assume this would normally be non-empty
    let capture = Capture {
        index: 1,
        name: Some(Box::new("test".into())),
        sub: Box::new(non_empty_hir),
    };
    let hir_with_capture = Hir {
        kind: HirKind::Capture(capture),
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let _ = hir_with_capture; // Test drop on this value, not hitting return on empty subs
}

