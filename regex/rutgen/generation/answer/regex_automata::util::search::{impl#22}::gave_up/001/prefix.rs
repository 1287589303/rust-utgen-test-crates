// Answer 0

#[test]
fn test_gave_up_with_zero_offset() {
    let error = MatchError::gave_up(0);
}

#[test]
fn test_gave_up_with_small_offset() {
    let error = MatchError::gave_up(5);
}

#[test]
fn test_gave_up_with_large_offset() {
    let error = MatchError::gave_up(usize::MAX);
}

