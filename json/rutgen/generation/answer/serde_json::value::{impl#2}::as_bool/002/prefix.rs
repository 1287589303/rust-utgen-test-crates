// Answer 0

#[test]
fn test_as_bool_true() {
    let value = Value::Bool(true);
    let result = value.as_bool();
}

#[test]
fn test_as_bool_false() {
    let value = Value::Bool(false);
    let result = value.as_bool();
}

