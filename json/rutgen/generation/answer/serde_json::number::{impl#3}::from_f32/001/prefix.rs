// Answer 0

#[test]
fn test_from_f32_positive_small() {
    let result = Number::from_f32(1.0_f32);
}

#[test]
fn test_from_f32_positive_large() {
    let result = Number::from_f32(3.4028234e38_f32);
}

#[test]
fn test_from_f32_zero() {
    let result = Number::from_f32(0.0_f32);
}

#[test]
fn test_from_f32_negative_small() {
    let result = Number::from_f32(-1.0_f32);
}

#[test]
fn test_from_f32_negative_large() {
    let result = Number::from_f32(-3.4028234e38_f32);
}

#[test]
fn test_from_f32_positive_subnormal() {
    let result = Number::from_f32(1.2e-38_f32);
}

#[test]
fn test_from_f32_negative_subnormal() {
    let result = Number::from_f32(-1.2e-38_f32);
}

