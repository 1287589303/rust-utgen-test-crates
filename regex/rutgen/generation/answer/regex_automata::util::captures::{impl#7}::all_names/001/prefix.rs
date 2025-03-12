// Answer 0

#[test]
fn test_all_names_with_no_patterns() {
    let group_info = GroupInfo::empty();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_single_pattern_no_captures() {
    let group_info = GroupInfo::new(vec![Some("a")]).unwrap();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_single_pattern_with_captures() {
    let group_info = GroupInfo::new(vec![Some("(?P<foo>a)")]).unwrap();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_multiple_patterns_mixed() {
    let group_info = GroupInfo::new(vec![
        Some("(?P<foo>a)"),
        Some("a"),
        Some("(a)"),
    ])
    .unwrap();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_multiple_patterns_all_no_captures() {
    let group_info = GroupInfo::new(vec![
        None,
        None,
        None,
    ])
    .unwrap();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_multiple_patterns_and_no_named_groups() {
    let group_info = GroupInfo::new(vec![
        Some("(a)"),
        Some("(b)"),
        Some("(c)"),
    ])
    .unwrap();
    let names = group_info.all_names();
}

#[test]
fn test_all_names_with_large_number_of_patterns() {
    let patterns: Vec<Option<&str>> = (0..10).map(|i| Some(format!("(pattern{})", i).as_str())).collect();
    let group_info = GroupInfo::new(patterns).unwrap();
    let names = group_info.all_names();
}

