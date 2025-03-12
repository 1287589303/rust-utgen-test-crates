// Answer 0

#[test]
fn test_one_of_fmt_two_names() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let formatter = &mut TestFormatter { output: String::new() };
    let one_of = OneOf { names: &["first_name", "second_name"] };
    let _ = one_of.fmt(formatter);
}

#[test]
#[should_panic]
fn test_one_of_fmt_empty_names() {
    struct TestFormatter {
        output: String,
    }

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let formatter = &mut TestFormatter { output: String::new() };
    let one_of = OneOf { names: &[] };
    let _ = one_of.fmt(formatter);
}

