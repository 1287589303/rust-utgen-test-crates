// Answer 0

#[test]
fn test_group_info_with_all_captures() {
    let group_info = GroupInfo::default();
    let captures = Captures::all(group_info.clone());
    let _ = captures.group_info();
}

#[test]
fn test_group_info_with_matches() {
    let group_info = GroupInfo::default();
    let captures = Captures::matches(group_info.clone());
    let _ = captures.group_info();
}

#[test]
fn test_group_info_with_empty_captures() {
    let group_info = GroupInfo::default();
    let captures = Captures::empty(group_info.clone());
    let _ = captures.group_info();
}

