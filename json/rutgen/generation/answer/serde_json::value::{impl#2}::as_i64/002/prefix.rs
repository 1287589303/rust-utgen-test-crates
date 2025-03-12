// Answer 0

#[test]
fn test_as_i64_with_valid_i64() {
    let value = Value::Number(Number::from_i128(-123).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_large_i128() {
    let value = Value::Number(Number::from_i128(i128::MAX).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_small_i128() {
    let value = Value::Number(Number::from_i128(i128::MIN).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_valid_u64() {
    let value = Value::Number(Number::from_u128(100).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_large_u64() {
    let value = Value::Number(Number::from_u128(u128::MAX).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_floating_point() {
    let value = Value::Number(Number::from_f64(1234.56).unwrap());
    let result = value.as_i64();
}

#[test]
fn test_as_i64_with_nan() {
    let value = Value::Number(Number::from_f64(f64::NAN).unwrap());
    let result = value.as_i64();
}

