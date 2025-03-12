// Answer 0

#[test]
fn test_capture_with_zero_index() {
    let sub_expression = Hir::empty();
    let capture = Capture {
        index: 0,
        name: None,
        sub: Box::new(sub_expression),
    };
    Hir::capture(capture);
}

#[test]
fn test_capture_with_non_empty_name() {
    let sub_expression = Hir::empty();
    let capture = Capture {
        index: 1,
        name: Some(Box::new("test".to_string())),
        sub: Box::new(sub_expression),
    };
    Hir::capture(capture);
}

#[test]
fn test_capture_with_large_index() {
    let sub_expression = Hir::literal("test");
    let capture = Capture {
        index: u32::MAX,
        name: None,
        sub: Box::new(sub_expression),
    };
    Hir::capture(capture);
}

#[test]
fn test_capture_with_empty_capture_name() {
    let sub_expression = Hir::fail();
    let capture = Capture {
        index: 2,
        name: Some(Box::new("".to_string())),
        sub: Box::new(sub_expression),
    };
    Hir::capture(capture);
}

#[test]
fn test_capture_with_empty_sub_expression() {
    let sub_expression = Hir::empty();
    let capture = Capture {
        index: 3,
        name: Some(Box::new("valid".to_string())),
        sub: Box::new(sub_expression),
    };
    Hir::capture(capture);
}

