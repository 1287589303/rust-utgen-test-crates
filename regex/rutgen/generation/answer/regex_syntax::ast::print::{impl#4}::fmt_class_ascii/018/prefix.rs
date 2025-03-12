// Answer 0

#[test]
fn test_fmt_class_ascii_digit_not_negated() {
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
        span: Span::default(), // Assuming a default Span implementation
        kind: ClassAsciiKind::Digit,
        negated: false,
    };
    
    writer.fmt_class_ascii(&ast).unwrap();
}

