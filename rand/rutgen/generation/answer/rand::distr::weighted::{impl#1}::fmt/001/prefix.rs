// Answer 0

#[test]
fn test_error_fmt_overflow_large_sum() {
    let error = Error::Overflow;
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_error_fmt_overflow_single_large_weight() {
    let error = Error::Overflow;
    let mut output = String::new();
    let _ = error.fmt(&mut output);
}

