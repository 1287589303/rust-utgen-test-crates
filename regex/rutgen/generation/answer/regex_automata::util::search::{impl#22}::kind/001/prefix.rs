// Answer 0

#[test]
fn test_kind_quit() {
    let error = MatchError::quit(42, 0);
    let kind = error.kind();
}

#[test]
fn test_kind_gave_up() {
    let error = MatchError::gave_up(0);
    let kind = error.kind();
}

#[test]
fn test_kind_haystack_too_long() {
    let error = MatchError::haystack_too_long(100);
    let kind = error.kind();
}

#[test]
fn test_kind_unsupported_anchored() {
    struct DummyAnchored;
    let error = MatchError::unsupported_anchored(DummyAnchored);
    let kind = error.kind();
}

#[test]
fn test_kind_quit_max_byte() {
    let error = MatchError::quit(255, 10);
    let kind = error.kind();
}

#[test]
fn test_kind_haystack_too_long_zero_len() {
    let error = MatchError::haystack_too_long(0);
    let kind = error.kind();
}

#[test]
fn test_kind_gave_up_large_offset() {
    let error = MatchError::gave_up(1000);
    let kind = error.kind();
}

