// Answer 0

#[test]
fn test_fmt_unsupported_anchored_yes() {
    let error = StartError::UnsupportedAnchored {
        mode: Anchored::Yes,
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_unsupported_anchored_no() {
    let error = StartError::UnsupportedAnchored {
        mode: Anchored::No,
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_unsupported_anchored_pattern() {
    let pid = PatternID(Default::default());
    let error = StartError::UnsupportedAnchored {
        mode: Anchored::Pattern(pid),
    };
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

