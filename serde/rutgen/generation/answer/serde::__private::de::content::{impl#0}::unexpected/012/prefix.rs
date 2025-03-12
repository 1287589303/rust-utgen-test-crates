// Answer 0

#[test]
fn test_unexpected_float_f64() {
    let content = Content::F64(0.0);
    let _ = content.unexpected(); // Test for zero

    let content = Content::F64(1.0);
    let _ = content.unexpected(); // Test for positive value

    let content = Content::F64(-1.0);
    let _ = content.unexpected(); // Test for negative value

    let content = Content::F64(f64::MAX);
    let _ = content.unexpected(); // Test for maximal f64

    let content = Content::F64(f64::MIN);
    let _ = content.unexpected(); // Test for minimal f64

    let content = Content::F64(f32::MAX as f64);
    let _ = content.unexpected(); // Test for maximal f32 represented in f64

    let content = Content::F64(f32::MIN as f64);
    let _ = content.unexpected(); // Test for minimal f32 represented in f64

    let content = Content::F64(f64::INFINITY);
    let _ = content.unexpected(); // Test for positive infinity

    let content = Content::F64(f64::NEG_INFINITY);
    let _ = content.unexpected(); // Test for negative infinity

    let content = Content::F64(f64::NAN);
    let _ = content.unexpected(); // Test for NaN
}

