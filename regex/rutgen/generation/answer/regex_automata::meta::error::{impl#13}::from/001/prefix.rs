// Answer 0

#[test]
fn test_retry_error_from_zero_offset() {
    let err = RetryFailError { offset: 0 };
    let result = RetryError::from(err);
}

#[test]
fn test_retry_error_from_one_offset() {
    let err = RetryFailError { offset: 1 };
    let result = RetryError::from(err);
}

#[test]
fn test_retry_error_from_max_usize_offset() {
    let err = RetryFailError { offset: std::usize::MAX };
    let result = RetryError::from(err);
}

