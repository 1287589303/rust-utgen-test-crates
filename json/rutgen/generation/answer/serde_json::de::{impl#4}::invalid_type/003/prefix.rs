// Answer 0

#[test]
fn test_invalid_type_f64_normal() {
    let value = ParserNumber::F64(1.23);
    let exp = &Unexpected::Float(0.0); // Replace with appropriate Expected implementation
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_subnormal() {
    let value = ParserNumber::F64(5e-324); // Smallest positive subnormal f64
    let exp = &Unexpected::Float(0.0); // Replace with appropriate Expected implementation
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_nan() {
    let value = ParserNumber::F64(f64::NAN);
    let exp = &Unexpected::Float(0.0); // Replace with appropriate Expected implementation
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_neg_infinity() {
    let value = ParserNumber::F64(f64::NEG_INFINITY);
    let exp = &Unexpected::Float(0.0); // Replace with appropriate Expected implementation
    value.invalid_type(exp);
}

#[test]
fn test_invalid_type_f64_pos_infinity() {
    let value = ParserNumber::F64(f64::INFINITY);
    let exp = &Unexpected::Float(0.0); // Replace with appropriate Expected implementation
    value.invalid_type(exp);
}

