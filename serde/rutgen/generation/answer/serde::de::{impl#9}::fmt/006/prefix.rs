// Answer 0

#[test]
fn test_with_nan() {
    use std::fmt::{self, Write};

    struct FakeFormatter {
        output: String,
    }

    impl fmt::Write for FakeFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = WithDecimalPoint(std::f64::NAN);
    let mut formatter = FakeFormatter { output: String::new() };
    let _ = value.fmt(&mut formatter);
    let _ = formatter.write_str(".0"); // Testing the condition
}

#[test]
fn test_with_infinity() {
    use std::fmt::{self, Write};

    struct FakeFormatter {
        output: String,
    }

    impl fmt::Write for FakeFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = WithDecimalPoint(std::f64::INFINITY);
    let mut formatter = FakeFormatter { output: String::new() };
    let _ = value.fmt(&mut formatter);
    let _ = formatter.write_str(".0"); // Testing the condition
}

#[test]
fn test_with_negative_infinity() {
    use std::fmt::{self, Write};

    struct FakeFormatter {
        output: String,
    }

    impl fmt::Write for FakeFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let value = WithDecimalPoint(std::f64::NEG_INFINITY);
    let mut formatter = FakeFormatter { output: String::new() };
    let _ = value.fmt(&mut formatter);
    let _ = formatter.write_str(".0"); // Testing the condition
}

