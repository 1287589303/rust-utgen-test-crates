// Answer 0

#[test]
fn test_display_alignment_mismatch() {
    struct TestError(DeserializeErrorKind);

    let alignment = 4; // Example alignment
    let address = alignment + 1; // Address not aligned to the alignment

    let error = TestError(DeserializeErrorKind::AlignmentMismatch {
        alignment,
        address,
    });

    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_display_alignment_mismatch_case_two() {
    struct TestError(DeserializeErrorKind);

    let alignment = 8; // Example alignment
    let address = alignment + 3; // Address not aligned to the alignment

    let error = TestError(DeserializeErrorKind::AlignmentMismatch {
        alignment,
        address,
    });

    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

