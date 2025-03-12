// Answer 0

#[test]
fn test_fmt_assertion_word_boundary_start() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryStart,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

#[test]
fn test_fmt_assertion_word_boundary_end() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

#[test]
fn test_fmt_assertion_word_boundary_start_angle() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

#[test]
fn test_fmt_assertion_word_boundary_end_angle() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryEndAngle,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

#[test]
fn test_fmt_assertion_word_boundary_start_half() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

#[test]
fn test_fmt_assertion_word_boundary_end_half() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = MockWriter { output: String::new() };
    let assertion = ast::Assertion {
        span: Span::new(0, 1),
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };
    let mut writer_instance = Writer { wtr: &mut writer };

    writer_instance.fmt_assertion(&assertion).unwrap();
}

