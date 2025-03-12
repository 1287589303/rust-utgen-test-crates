// Answer 0

#[test]
fn test_fmt_assertion_start_line() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_end_line() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndLine,
    };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_start_text() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_end_text() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };
    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };
    let _ = writer.fmt_assertion(&assertion);
}

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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };
    let _ = writer.fmt_assertion(&assertion);
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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };
    let _ = writer.fmt_assertion(&assertion);
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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };
    let _ = writer.fmt_assertion(&assertion);
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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndAngle,
    };
    let _ = writer.fmt_assertion(&assertion);
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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };
    let _ = writer.fmt_assertion(&assertion);
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

    let mut writer = Writer { wtr: MockWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };
    let _ = writer.fmt_assertion(&assertion);
}

