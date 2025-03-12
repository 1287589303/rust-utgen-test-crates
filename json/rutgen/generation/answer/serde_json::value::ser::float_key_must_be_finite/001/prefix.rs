// Answer 0

#[test]
fn test_float_key_must_be_finite() {
    let result = float_key_must_be_finite();
}

#[test]
fn test_float_key_must_be_finite_error_code() {
    let result = float_key_must_be_finite();
    match result.err.code {
        ErrorCode::FloatKeyMustBeFinite => {},
        _ => panic!("Expected FloatKeyMustBeFinite"),
    }
}

#[test]
fn test_float_key_must_be_finite_line() {
    let result = float_key_must_be_finite();
    assert_eq!(result.err.line, 0);
}

#[test]
fn test_float_key_must_be_finite_column() {
    let result = float_key_must_be_finite();
    assert_eq!(result.err.column, 0);
}

