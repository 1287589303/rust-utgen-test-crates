// Answer 0

#[test]
fn test_unexpected_bool_true() {
    let value = Value::Bool(true);
    let result = value.unexpected();
}

#[test]
fn test_unexpected_bool_false() {
    let value = Value::Bool(false);
    let result = value.unexpected();
}

