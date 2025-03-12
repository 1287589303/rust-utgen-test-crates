// Answer 0

#[test]
fn test_source_with_captures_error() {
    let group_info_error = captures::GroupInfoError { /* initialize with valid data */ };
    let build_error = BuildError::captures(group_info_error.clone());
    assert!(matches!(build_error.kind(), BuildErrorKind::Captures(_)));
    let result = build_error.source();
    let expected = Some(&group_info_error);
    result.unwrap(); // this is to ensure the value is present
}

#[test]
fn test_source_with_another_captures_error() {
    let group_info_error = captures::GroupInfoError { /* initialize with valid data */ };
    let build_error = BuildError::captures(group_info_error.clone());
    assert!(matches!(build_error.kind(), BuildErrorKind::Captures(_)));
    let result = build_error.source();
    let expected = Some(&group_info_error);
    result.unwrap(); // this is to ensure the value is present
}

