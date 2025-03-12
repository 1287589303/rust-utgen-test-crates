// Answer 0

#[test]
fn test_group_info_valid_input() {
    let result = GroupInfo::new(vec![
        vec![None, Some("group1"), Some("group2")],
        vec![None, Some("groupA")],
        vec![None, None, Some("groupB")],
    ]);
    let _ = result.unwrap();
}

#[test]
fn test_group_info_empty_input() {
    let result = GroupInfo::new(Vec::<Vec<Option<String>>>::new());
    let _ = result.unwrap();
}

#[test]
#[should_panic]
fn test_group_info_missing_groups() {
    let _ = GroupInfo::new(vec![
        vec![None, Some("group1")],
        vec![],
    ]);
}

#[test]
#[should_panic]
fn test_group_info_first_group_named() {
    let _ = GroupInfo::new(vec![
        vec![Some("group3")],
    ]);
}

#[test]
#[should_panic]
fn test_group_info_duplicate_group_names() {
    let _ = GroupInfo::new(vec![
        vec![None, Some("groupX"), Some("groupX")],
    ]);
}

#[test]
fn test_group_info_duplicate_names_across_patterns() {
    let result = GroupInfo::new(vec![
        vec![None, Some("groupX")],
        vec![None, Some("groupX")],
    ]);
    let _ = result.unwrap();
}

#[test]
#[should_panic]
fn test_group_info_too_many_patterns() {
    let mut patterns = Vec::new();
    for i in 0..(u32::MAX as usize) {
        patterns.push(vec![None, Some("name")]);
    }
    let _ = GroupInfo::new(patterns);
}

