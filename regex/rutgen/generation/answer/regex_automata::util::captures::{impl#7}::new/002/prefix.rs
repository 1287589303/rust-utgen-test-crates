// Answer 0

#[test]
fn test_group_info_first_group_must_be_unnamed() {
    let result = GroupInfo::new(vec![
        vec![Some("named_group")],
        vec![None],
    ]);
    let _ = result.unwrap_err(); // Expected to trigger Err(GroupInfoError::first_must_be_unnamed)
}

#[test]
fn test_group_info_first_group_must_be_unnamed_multiple_patterns() {
    let result = GroupInfo::new(vec![
        vec![Some("first_group"), Some("second_group")],
        vec![None, None],
    ]);
    let _ = result.unwrap_err(); // Expected to trigger Err(GroupInfoError::first_must_be_unnamed)
}

#[test]
fn test_group_info_first_group_must_be_unnamed_with_empty_groups() {
    let result = GroupInfo::new(vec![
        vec![Some("first_named")],
        vec![None, Some("second_named")],
    ]);
    let _ = result.unwrap_err(); // Expected to trigger Err(GroupInfoError::first_must_be_unnamed)
}

