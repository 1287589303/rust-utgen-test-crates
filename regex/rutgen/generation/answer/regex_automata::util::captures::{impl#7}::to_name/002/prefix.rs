// Answer 0

#[test]
fn test_to_name_valid_pid_invalid_group_index() {
    let pid = PatternID(SmallIndex::new(0));
    let group_index = 5;

    let group_info = GroupInfo(GroupInfoInner {
        index_to_name: vec![
            vec![Some(Arc::from("foo")), Some(Arc::from("bar"))],
            vec![Some(Arc::from("baz"))],
        ],
        ..Default::default()
    });

    let result = group_info.to_name(pid, group_index);
}

#[test]
fn test_to_name_valid_pid_below_group_index() {
    let pid = PatternID(SmallIndex::new(1));
    let group_index = 1;

    let group_info = GroupInfo(GroupInfoInner {
        index_to_name: vec![
            vec![Some(Arc::from("foo"))],
            vec![Some(Arc::from("baz")), None],
        ],
        ..Default::default()
    });

    let result = group_info.to_name(pid, group_index);
}

#[test]
fn test_to_name_valid_pid_empty_group_names() {
    let pid = PatternID(SmallIndex::new(0));
    let group_index = 2;

    let group_info = GroupInfo(GroupInfoInner {
        index_to_name: vec![
            vec![Some(Arc::from("foo")), Some(Arc::from("bar"))],
        ],
        ..Default::default()
    });

    let result = group_info.to_name(pid, group_index);
}

