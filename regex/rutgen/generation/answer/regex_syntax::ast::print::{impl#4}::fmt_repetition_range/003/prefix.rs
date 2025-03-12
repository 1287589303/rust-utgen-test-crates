// Answer 0

#[test]
fn test_fmt_repetition_range_exactly_0() {
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
    let ast = ast::RepetitionRange::Exactly(0);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly_1() {
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
    let ast = ast::RepetitionRange::Exactly(1);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly_2() {
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
    let ast = ast::RepetitionRange::Exactly(2);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly_100() {
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
    let ast = ast::RepetitionRange::Exactly(100);
    writer.fmt_repetition_range(&ast).unwrap();
}

#[test]
fn test_fmt_repetition_range_exactly_u32_max() {
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
    let ast = ast::RepetitionRange::Exactly(u32::MAX);
    writer.fmt_repetition_range(&ast).unwrap();
}

