// Answer 0

#[test]
fn test_group_info_new_valid() {
    let patterns = vec![
        vec![None, Some("group1")],
        vec![None, Some("group2")],
    ];
    let _ = GroupInfo::new(patterns);
}

#[test]
fn test_group_info_new_empty() {
    let patterns: Vec<Vec<Option<String>>> = Vec::new();
    let _ = GroupInfo::new(patterns);
}

#[test]
fn test_group_info_new_first_group_named() {
    let patterns = vec![
        vec![Some("group1")],
        vec![None],
    ];
    let _ = GroupInfo::new(patterns).err().unwrap();
}

#[test]
fn test_group_info_new_missing_groups() {
    let patterns = vec![
        vec![None],
        vec![],
    ];
    let _ = GroupInfo::new(patterns).err().unwrap();
}

#[test]
fn test_group_info_new_duplicate_names() {
    let patterns = vec![
        vec![None, Some("group1"), Some("group1")],
    ];
    let _ = GroupInfo::new(patterns).err().unwrap();
}

#[test]
fn test_group_info_new_duplicate_names_across_patterns() {
    let patterns = vec![
        vec![None, Some("group1")],
        vec![None, Some("group1")],
    ];
    let _ = GroupInfo::new(patterns);
}

#[test]
fn test_group_info_new_too_many_groups() {
    let patterns = vec![
        vec![None],
        (0..=u32::MAX).map(|_| Some("group")).collect::<Vec<_>>(),
    ];
    let _ = GroupInfo::new(patterns).err().unwrap();
}

