// Answer 0

#[test]
fn test_match_error_new_quit() {
    let byte = 128; // Example byte in the valid range [0, 255]
    let offset = 42; // Example offset >= 0
    let kind = MatchErrorKind::Quit { byte, offset };
    let _error = MatchError::new(kind);
}

#[test]
fn test_match_error_new_gave_up() {
    let offset = 10; // Example offset >= 0
    let kind = MatchErrorKind::GaveUp { offset };
    let _error = MatchError::new(kind);
}

#[test]
fn test_match_error_new_haystack_too_long() {
    let len = 100; // Example length > 0
    let kind = MatchErrorKind::HaystackTooLong { len };
    let _error = MatchError::new(kind);
}

#[test]
fn test_match_error_new_unsupported_anchored() {
    struct DummyAnchored;
    let mode = DummyAnchored; // Assuming DummyAnchored is a valid Anchored type
    let kind = MatchErrorKind::UnsupportedAnchored { mode };
    let _error = MatchError::new(kind);
}

