// Answer 0

#[test]
fn test_fmt_assertion_end_text() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::EndText,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_start_line() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartLine,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_start_text() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::StartText,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundary,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_not_word_boundary() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::NotWordBoundary,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStart,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEnd,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start_angle() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end_angle() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndAngle,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_start_half() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };

    let _ = writer.fmt_assertion(&assertion);
}

#[test]
fn test_fmt_assertion_word_boundary_end_half() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = Writer { wtr: TestWriter { output: String::new() } };
    let assertion = ast::Assertion {
        span: Span::default(),
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };

    let _ = writer.fmt_assertion(&assertion);
}

