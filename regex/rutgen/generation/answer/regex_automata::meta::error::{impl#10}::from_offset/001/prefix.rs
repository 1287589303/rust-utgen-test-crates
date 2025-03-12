// Answer 0

#[test]
fn test_from_offset_zero() {
    let error = RetryFailError::from_offset(0);
}

#[test]
fn test_from_offset_one() {
    let error = RetryFailError::from_offset(1);
}

#[test]
fn test_from_offset_two() {
    let error = RetryFailError::from_offset(2);
}

#[test]
fn test_from_offset_max() {
    let error = RetryFailError::from_offset(usize::MAX);
}

