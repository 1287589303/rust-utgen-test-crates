// Answer 0

#[test]
fn test_jsonunexpected_float() {
    use serde::de;
    use core::fmt;
    
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let float_values: [f64; 5] = [0.0, -1.0, 1.0, f64::INFINITY, f64::NAN];

    for &value in &float_values {
        let unexpected_float = de::Unexpected::Float(value);
        let json_unexpected = JsonUnexpected(unexpected_float);
        let mut formatter = TestFormatter;

        json_unexpected.fmt(&mut formatter);
    }
}

