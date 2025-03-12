// Answer 0

#[test]
fn test_value_unexpected_with_integer_number() {
    struct IntegerNumber;
    impl Number for IntegerNumber {}
    
    let number_instance = IntegerNumber;
    let value_instance = Value::Number(number_instance);
    value_instance.unexpected();
}

#[test]
fn test_value_unexpected_with_float_number() {
    struct FloatNumber;
    impl Number for FloatNumber {}
    
    let number_instance = FloatNumber;
    let value_instance = Value::Number(number_instance);
    value_instance.unexpected();
}

#[test]
fn test_value_unexpected_with_negative_number() {
    struct NegativeNumber;
    impl Number for NegativeNumber {}
    
    let number_instance = NegativeNumber;
    let value_instance = Value::Number(number_instance);
    value_instance.unexpected();
}

#[test]
fn test_value_unexpected_with_nan() {
    struct NaNNumber;
    impl Number for NaNNumber {}
    
    let number_instance = NaNNumber;
    let value_instance = Value::Number(number_instance);
    value_instance.unexpected();
}

#[test]
fn test_value_unexpected_with_infinity() {
    struct InfinityNumber;
    impl Number for InfinityNumber {}
    
    let number_instance = InfinityNumber;
    let value_instance = Value::Number(number_instance);
    value_instance.unexpected();
}

