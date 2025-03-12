// Answer 0

#[test]
fn test_is_f64_valid_f64() {
    let float_value = Value::Number(Number::from_f64(0.0).unwrap());
    float_value.is_f64();

    let float_value = Value::Number(Number::from_f64(1.0).unwrap());
    float_value.is_f64();

    let float_value = Value::Number(Number::from_f64(-1.0).unwrap());
    float_value.is_f64();

    let float_value = Value::Number(Number::from_f64(std::f64::MAX).unwrap());
    float_value.is_f64();

    let float_value = Value::Number(Number::from_f64(std::f64::MIN).unwrap());
    float_value.is_f64();
}

#[test]
fn test_is_f64_valid_integer() {
    let int_value = Value::Number(Number::from_i128(1).unwrap());
    int_value.is_f64();

    let int_value = Value::Number(Number::from_i128(2_147_483_647).unwrap());
    int_value.is_f64();

    let int_value = Value::Number(Number::from_i128(-2_147_483_648).unwrap());
    int_value.is_f64();
}

#[test]
fn test_is_f64_edge_cases() {
    let nan_value = Value::Number(Number::from_f64(std::f64::NAN).unwrap());
    nan_value.is_f64();

    let infinity_value = Value::Number(Number::from_f64(std::f64::INFINITY).unwrap());
    infinity_value.is_f64();

    let neg_infinity_value = Value::Number(Number::from_f64(std::f64::NEG_INFINITY).unwrap());
    neg_infinity_value.is_f64();
}

#[test]
fn test_is_f64_invalid_cases() {
    let int_value = Value::Number(Number::from_i64(64).unwrap());
    int_value.is_f64();

    let negative_int_value = Value::Number(Number::from_i64(-64).unwrap());
    negative_int_value.is_f64();
}

