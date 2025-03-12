// Answer 0

#[test]
fn test_fmt_class_ascii_alnum_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Alnum,
        negated: true,
    };
    
    Writer { wtr: &mut writer }.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_alpha_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Alpha,
        negated: true,
    };

    Writer { wtr: &mut writer }.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_digit_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Digit,
        negated: true,
    };

    Writer { wtr: &mut writer }.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_space_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Space,
        negated: true,
    };

    Writer { wtr: &mut writer }.fmt_class_ascii(&ast).unwrap();
}

#[test]
fn test_fmt_class_ascii_upper_negated() {
    struct TestWriter {
        output: String,
    }

    impl fmt::Write for TestWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut writer = TestWriter { output: String::new() };
    let ast = ast::ClassAscii {
        span: Default::default(),
        kind: ast::ClassAsciiKind::Upper,
        negated: true,
    };

    Writer { wtr: &mut writer }.fmt_class_ascii(&ast).unwrap();
}

