// Answer 0

#[test]
fn test_is_i64_positive_boundary() {
    let num = Number::from_i128(9223372036854775807).unwrap(); // i64::MAX
    let value = Value::Number(num);
    value.is_i64();
}

#[test]
fn test_is_i64_negative_boundary() {
    let num = Number::from_i128(-9223372036854775808).unwrap(); // i64::MIN
    let value = Value::Number(num);
    value.is_i64();
}

#[test]
fn test_is_i64_with_valid_i64() {
    let num = Number::from_i128(123456789).unwrap(); // a valid i64 value
    let value = Value::Number(num);
    value.is_i64();
}

#[test]
fn test_is_i64_with_valid_negative_i64() {
    let num = Number::from_i128(-123456789).unwrap(); // a valid negative i64 value
    let value = Value::Number(num);
    value.is_i64();
}

#[test]
fn test_is_i64_with_valid_zero() {
    let num = Number::from_i128(0).unwrap(); // zero is also a valid i64
    let value = Value::Number(num);
    value.is_i64();
}

