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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::StartLine,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::EndLine,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::StartText,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::EndText,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundary,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::NotWordBoundary,
    };
    writer.fmt_assertion(&ast).unwrap();
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

    let mut writer = MockWriter { output: String::new() };
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryStart,
    };
    writer.fmt_assertion(&ast).unwrap();
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
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryEnd,
    };
    writer.fmt_assertion(&ast).unwrap();
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
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryStartAngle,
    };
    writer.fmt_assertion(&ast).unwrap();
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
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryEndAngle,
    };
    writer.fmt_assertion(&ast).unwrap();
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
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryStartHalf,
    };
    writer.fmt_assertion(&ast).unwrap();
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
    let ast = ast::Assertion {
        span: Span {},
        kind: ast::AssertionKind::WordBoundaryEndHalf,
    };
    writer.fmt_assertion(&ast).unwrap();
}

