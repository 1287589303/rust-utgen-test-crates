// Answer 0

#[test]
fn test_format_quit_error() {
    let byte: u8 = 100; // example byte value
    let offset: usize = 500; // example offset
    let kind = MatchErrorKind::Quit { byte, offset };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

#[test]
fn test_format_gave_up_error() {
    let offset: usize = 300; // example offset
    let kind = MatchErrorKind::GaveUp { offset };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

#[test]
fn test_format_haystack_too_long() {
    let len: usize = 1024; // example length
    let kind = MatchErrorKind::HaystackTooLong { len };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

#[test]
fn test_format_unsupported_anchored_yes() {
    let kind = MatchErrorKind::UnsupportedAnchored { mode: Anchored::Yes };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

#[test]
fn test_format_unsupported_anchored_no() {
    let kind = MatchErrorKind::UnsupportedAnchored { mode: Anchored::No };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

#[test]
fn test_format_unsupported_anchored_pattern() {
    let pid = PatternID(0); // example PatternID
    let kind = MatchErrorKind::UnsupportedAnchored { mode: Anchored::Pattern(pid) };
    let error = MatchError::new(kind);
    let mut fmt = core::fmt::Formatter::new();
    let _ = error.fmt(&mut fmt);
}

