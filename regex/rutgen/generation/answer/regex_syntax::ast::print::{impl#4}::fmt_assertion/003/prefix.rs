// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_end_line() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndLine,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_start_text() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_end_text() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start_angle() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end_angle() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndAngle,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start_half() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end_half() {
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };
    let mut writer = Writer { wtr: String::new() };
    let _ = writer.fmt_assertion(&assertion);
}

