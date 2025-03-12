// Answer 0

#[test]
fn test_display_label_mismatch() {
    let error = DeserializeError(DeserializeErrorKind::LabelMismatch {
        expected: "expected_label\0",
    });
    let mut buffer = core::fmt::Formatter::default();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_display_label_mismatch_with_different_label() {
    let error = DeserializeError(DeserializeErrorKind::LabelMismatch {
        expected: "another_expected_label\0",
    });
    let mut buffer = core::fmt::Formatter::default();
    let _ = error.fmt(&mut buffer);
}

