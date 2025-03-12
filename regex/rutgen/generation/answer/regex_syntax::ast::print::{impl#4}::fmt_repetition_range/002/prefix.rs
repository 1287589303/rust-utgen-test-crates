// Answer 0

#[test]
fn test_fmt_repetition_range_at_least_zero() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let at_least_zero = ast::RepetitionRange::AtLeast(0);
    let result = mock_writer.fmt_repetition_range(&at_least_zero);
}

#[test]
fn test_fmt_repetition_range_at_least_one() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let at_least_one = ast::RepetitionRange::AtLeast(1);
    let result = mock_writer.fmt_repetition_range(&at_least_one);
}

#[test]
fn test_fmt_repetition_range_bounded_zero_to_one() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let bounded_zero_to_one = ast::RepetitionRange::Bounded(0, 1);
    let result = mock_writer.fmt_repetition_range(&bounded_zero_to_one);
}

#[test]
fn test_fmt_repetition_range_bounded_one_to_two() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let bounded_one_to_two = ast::RepetitionRange::Bounded(1, 2);
    let result = mock_writer.fmt_repetition_range(&bounded_one_to_two);
}

#[test]
fn test_fmt_repetition_range_bounded_three_to_five() {
    struct MockWriter {
        output: String,
    }

    impl fmt::Write for MockWriter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut mock_writer = MockWriter { output: String::new() };
    let bounded_three_to_five = ast::RepetitionRange::Bounded(3, 5);
    let result = mock_writer.fmt_repetition_range(&bounded_three_to_five);
}

