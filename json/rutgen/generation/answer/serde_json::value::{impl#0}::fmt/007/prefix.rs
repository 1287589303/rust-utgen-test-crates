// Answer 0

#[test]
fn test_value_bool_true() {
    let value = Value::Bool(true);
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

#[test]
fn test_value_bool_false() {
    let value = Value::Bool(false);
    let mut formatter = fmt::Formatter::new();
    value.fmt(&mut formatter);
}

