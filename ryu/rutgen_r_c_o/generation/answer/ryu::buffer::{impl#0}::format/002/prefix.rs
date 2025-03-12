// Answer 0

#[test]
fn test_format_finite_positive() {
    let mut buffer = Buffer::new();
    let value: f64 = 3.14159; 
    buffer.format(value);
}

#[test]
fn test_format_finite_negative() {
    let mut buffer = Buffer::new();
    let value: f64 = -3.14159; 
    buffer.format(value);
}

#[test]
fn test_format_finite_zero() {
    let mut buffer = Buffer::new();
    let value: f64 = 0.0; 
    buffer.format(value);
}

#[test]
fn test_format_finite_large_positive() {
    let mut buffer = Buffer::new();
    let value: f64 = 1.7976931348623157E+308; 
    buffer.format(value);
}

#[test]
fn test_format_finite_large_negative() {
    let mut buffer = Buffer::new();
    let value: f64 = -1.7976931348623157E+308; 
    buffer.format(value);
}

#[test]
fn test_format_finite_mid_range() {
    let mut buffer = Buffer::new();
    let value: f64 = 123456789.123456789; 
    buffer.format(value);
}

