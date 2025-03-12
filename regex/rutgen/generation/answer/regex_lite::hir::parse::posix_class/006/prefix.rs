// Answer 0

#[test]
fn test_posix_class_digit() {
    let kind = "digit";
    let result = posix_class(kind);
    let expected_ranges = vec![
        hir::ClassRange { start: '0', end: '9' },
    ];
    let _ = result.unwrap().collect::<Vec<_>>();
}

#[test]
fn test_posix_class_invalid() {
    let kind = "nonexistent";
    let result = posix_class(kind);
    let _ = result.unwrap_err();
}

