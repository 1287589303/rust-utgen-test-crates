// Answer 0

#[test]
fn test_splat_negative_infinity() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(f64::NEG_INFINITY);
    let result = FloatWrapper::splat(input.0);
}

#[test]
fn test_splat_zero() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(0.0);
    let result = FloatWrapper::splat(input.0);
}

#[test]
fn test_splat_positive_infinity() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(f64::INFINITY);
    let result = FloatWrapper::splat(input.0);
}

#[test]
fn test_splat_nan() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(f64::NAN);
    let result = FloatWrapper::splat(input.0);
}

#[test]
fn test_splat_negative_one() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(-1.0);
    let result = FloatWrapper::splat(input.0);
}

#[test]
fn test_splat_one() {
    struct FloatWrapper(f64);
    
    let input = FloatWrapper(1.0);
    let result = FloatWrapper::splat(input.0);
}

