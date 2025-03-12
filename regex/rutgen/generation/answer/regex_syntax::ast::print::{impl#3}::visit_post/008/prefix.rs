// Answer 0

#[test]
fn test_visit_post_with_start_line_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_with_end_line_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndLine,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_with_start_text_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_with_end_text_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_with_word_boundary_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

#[test]
fn test_visit_post_with_not_word_boundary_assertion() {
    struct MockWriter;

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut writer = MockWriter;
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let ast = Ast::Assertion(Box::new(assertion));

    let mut visitor = Writer { wtr: &mut writer };
    let _ = visitor.visit_post(&ast);
}

