// Answer 0

#[test]
fn test_eq_i64_with_valid_i64_value() {
    let number_value = Value::Number(Number::from_i64(123).unwrap());
    let result = eq_i64(&number_value, 123);
}

#[test]
fn test_eq_i64_with_negative_i64_boundary() {
    let number_value = Value::Number(Number::from_i64(-9223372036854775808).unwrap());
    let result = eq_i64(&number_value, -9223372036854775808);
}

#[test]
fn test_eq_i64_with_positive_i64_boundary() {
    let number_value = Value::Number(Number::from_i64(9223372036854775807).unwrap());
    let result = eq_i64(&number_value, 9223372036854775807);
}

#[test]
fn test_eq_i64_with_i64_overflow() {
    let number_value = Value::Number(Number::from_f64(9223372036854775808.0).unwrap()); // Outside i64 range
    let result = eq_i64(&number_value, 9223372036854775807);
}

#[test]
fn test_eq_i64_with_null() {
    let null_value = Value::Null;
    let result = eq_i64(&null_value, 123);
}

#[test]
fn test_eq_i64_with_bool() {
    let bool_value = Value::Bool(true);
    let result = eq_i64(&bool_value, 1);
}

#[test]
fn test_eq_i64_with_string() {
    let string_value = Value::String(String::from("test"));
    let result = eq_i64(&string_value, 123);
}

#[test]
fn test_eq_i64_with_array() {
    let array_value = Value::Array(vec![Value::Number(Number::from_i64(1).unwrap())]);
    let result = eq_i64(&array_value, 1);
}

#[test]
fn test_eq_i64_with_object() {
    let object_value = Value::Object(Map::new());
    let result = eq_i64(&object_value, 123);
}

