// Answer 0

#[test]
fn test_source_quit_case() {
    let start_error = StartError::Quit { byte: 0xFF };
    let result = start_error.source();
}

#[test]
fn test_source_unsupported_anchored_case() {
    let anchored_mode = Anchored::No;
    let start_error = StartError::UnsupportedAnchored { mode: anchored_mode };
    let result = start_error.source();
}

