// Answer 0

#[test]
fn test_from_quit_error_with_zero_offset() {
    let merr = MatchError::quit(0u8, 0);
    let error: RetryFailError = merr.into();
}

#[test]
fn test_from_quit_error_with_mid_range_offset() {
    let merr = MatchError::quit(128u8, 128);
    let error: RetryFailError = merr.into();
}

#[test]
fn test_from_quit_error_with_max_offset() {
    let merr = MatchError::quit(255u8, usize::MAX);
    let error: RetryFailError = merr.into();
}

