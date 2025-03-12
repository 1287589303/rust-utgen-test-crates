// Answer 0

#[test]
fn test_empty_captures_with_valid_group_info() {
    let group_info = GroupInfo::default();
    let captures = Captures::empty(group_info);
}

#[test]
fn test_empty_captures_with_custom_group_info() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner::default()));
    let captures = Captures::empty(group_info);
}

#[test]
fn test_empty_captures_boundary_empty_group_info() {
    let group_info = GroupInfo(Arc::new(GroupInfoInner::new_empty()));
    let captures = Captures::empty(group_info);
}

