// Answer 0

#[test]
fn test_capture_start_anchored_match_non_empty_with_some_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: Some(1),
    };
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_start_anchored_match_non_empty_with_none_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let capture = Capture {
        index: 1,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_start_anchored_match_empty_with_some_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: Some(2),
    };
    let capture = Capture {
        index: 2,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_start_anchored_match_empty_with_none_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: true,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    let capture = Capture {
        index: 3,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_not_start_anchored_match_non_empty_with_some_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: Some(0),
    };
    let capture = Capture {
        index: 4,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_not_start_anchored_match_non_empty_with_none_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: false,
        static_explicit_captures_len: None,
    };
    let capture = Capture {
        index: 5,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_not_start_anchored_match_empty_with_some_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: Some(3),
    };
    let capture = Capture {
        index: 6,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

#[test]
fn test_capture_not_start_anchored_match_empty_with_none_length() {
    let sub_hir = Hir {
        kind: HirKind::Empty,
        is_start_anchored: false,
        is_match_empty: true,
        static_explicit_captures_len: None,
    };
    let capture = Capture {
        index: 7,
        name: None,
        sub: Box::new(sub_hir),
    };
    let result = Hir::capture(capture);
}

