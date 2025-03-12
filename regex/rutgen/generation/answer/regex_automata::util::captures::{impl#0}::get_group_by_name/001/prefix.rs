// Answer 0

#[test]
fn test_get_group_by_name_empty() {
    let group_info = GroupInfo::empty();
    let captures = Captures::empty(group_info);
    let result = captures.get_group_by_name("non_existent");
}

#[test]
fn test_get_group_by_name_matches_invalid_index() {
    let group_info = GroupInfo::empty();
    let captures = Captures::matches(group_info);
    let result = captures.get_group_by_name("non_existent");
}

#[test]
fn test_get_group_by_name_matches_existing_name_invalid() {
    let group_info = {
        let mut map = std::collections::HashMap::new();
        map.insert("first".to_string(), 0);
        map.insert("last".to_string(), 1);
        GroupInfo::new(map).unwrap()
    };
    let captures = Captures::matches(group_info);
    let result = captures.get_group_by_name("middle");
}

