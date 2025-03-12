// Answer 0

#[test]
fn test_from_f32_nan() {
    let value = f32::NAN;
    let result = Number::from_f32(value);
}

#[test]
fn test_from_f32_negative_infinity() {
    let value = f32::NEG_INFINITY;
    let result = Number::from_f32(value);
}

#[test]
fn test_from_f32_positive_infinity() {
    let value = f32::INFINITY;
    let result = Number::from_f32(value);
}

