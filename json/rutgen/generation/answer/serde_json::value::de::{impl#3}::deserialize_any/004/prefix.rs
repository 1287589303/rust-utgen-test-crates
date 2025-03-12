// Answer 0

#[test]
fn test_deserialize_any_number_positive_integer() {
    let number_value = Value::Number(Number { n: 42 });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_negative_integer() {
    let number_value = Value::Number(Number { n: -42 });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_zero() {
    let number_value = Value::Number(Number { n: 0 });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_positive_float() {
    let number_value = Value::Number(Number { n: 12.5 });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_negative_float() {
    let number_value = Value::Number(Number { n: -12.5 });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_nan() {
    let number_value = Value::Number(Number { n: f64::NAN });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_infinity() {
    let number_value = Value::Number(Number { n: f64::INFINITY });
    number_value.deserialize_any(visitor);
}

#[test]
fn test_deserialize_any_number_negative_infinity() {
    let number_value = Value::Number(Number { n: f64::NEG_INFINITY });
    number_value.deserialize_any(visitor);
}

