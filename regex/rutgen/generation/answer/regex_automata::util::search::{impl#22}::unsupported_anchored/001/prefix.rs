// Answer 0

#[test]
fn test_unsupported_anchored_no() {
    let mode = Anchored::No;
    let error = MatchError::unsupported_anchored(mode);
}

#[test]
fn test_unsupported_anchored_yes() {
    let mode = Anchored::Yes;
    let error = MatchError::unsupported_anchored(mode);
}

#[test]
fn test_unsupported_anchored_pattern_valid() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct DummyPatternID(u32);
    
    let mode = Anchored::Pattern(DummyPatternID(1));
    let error = MatchError::unsupported_anchored(mode);
}

#[test]
#[should_panic]
fn test_unsupported_anchored_pattern_invalid() {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct DummyPatternID(u32);
    
    let mode = Anchored::Pattern(DummyPatternID(u32::MAX));
    let error = MatchError::unsupported_anchored(mode);
}

