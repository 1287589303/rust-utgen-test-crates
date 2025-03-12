// Answer 0

#[test]
fn test_format_nonfinite_negative_infinity() {
    let value: f64 = -1.0 / 0.0; 
    let result = value.format_nonfinite();
}

#[test]
fn test_format_nonfinite_negative_infinity_alternate() {
    let value: f64 = f64::NEG_INFINITY; 
    let result = value.format_nonfinite();
}

