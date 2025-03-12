// Answer 0

#[test]
fn test_from_match_error_gave_up_zero_offset() {
    let merr = MatchError::gave_up(0);
    let retry_fail_error: RetryFailError = merr.into();
}

#[test]
fn test_from_match_error_gave_up_max_offset() {
    let merr = MatchError::gave_up(usize::MAX);
    let retry_fail_error: RetryFailError = merr.into();
}

#[test]
fn test_from_match_error_gave_up_mid_offset() {
    let merr = MatchError::gave_up(12345);
    let retry_fail_error: RetryFailError = merr.into();
}

