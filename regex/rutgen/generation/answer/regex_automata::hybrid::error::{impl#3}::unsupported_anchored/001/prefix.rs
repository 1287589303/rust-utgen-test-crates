// Answer 0

#[test]
fn test_unsupported_anchored_no() {
    let mode = Anchored::No;
    let error = StartError::unsupported_anchored(mode);
}

#[test]
fn test_unsupported_anchored_yes() {
    let mode = Anchored::Yes;
    let error = StartError::unsupported_anchored(mode);
}

#[test]
fn test_unsupported_anchored_pattern() {
    struct PatternID;
    let mode = Anchored::Pattern(PatternID);
    let error = StartError::unsupported_anchored(mode);
}

