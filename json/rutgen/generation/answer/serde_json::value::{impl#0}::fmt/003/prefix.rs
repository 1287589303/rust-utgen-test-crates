// Answer 0

#[test]
fn test_fmt_array_empty() {
    let value = Value::Array(Vec::new());
    let mut formatter = fmt::Formatter::default();
    let _ = value.fmt(&mut formatter);
}

#[test]
fn test_fmt_array_with_err() {
    struct ErrFormatter;

    impl fmt::Write for ErrFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Err(fmt::Error)
        }
    }

    let value = Value::Array(vec![Value::String("test".to_string())]);
    let mut formatter = ErrFormatter;
    let _ = value.fmt(&mut formatter);
}

