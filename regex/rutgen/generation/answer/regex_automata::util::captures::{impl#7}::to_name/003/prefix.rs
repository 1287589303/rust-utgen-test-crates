// Answer 0

#[test]
fn test_to_name_valid_case() {
    let group_info = GroupInfo::new(vec![
        vec![Some(Arc::from("foo")), Some(Arc::from("bar"))],
        vec![None, Some(Arc::from("baz")), Some(Arc::from("qux"))],
    ]).unwrap();

    let pid = PatternID::must(0);
    let group_index = 1;

    group_info.to_name(pid, group_index);
}

#[test]
fn test_to_name_empty_capture() {
    let group_info = GroupInfo::empty();

    let pid = PatternID::must(0);
    let group_index = 0;

    group_info.to_name(pid, group_index);
}

#[test]
fn test_to_name_out_of_range_index() {
    let group_info = GroupInfo::new(vec![
        vec![Some(Arc::from("foo"))],
    ]).unwrap();

    let pid = PatternID::must(0);
    let group_index = 1;

    group_info.to_name(pid, group_index);
}

#[test]
fn test_to_name_valid_second_pattern() {
    let group_info = GroupInfo::new(vec![
        vec![None, Some(Arc::from("foo")), None],
        vec![Some(Arc::from("bar")), Some(Arc::from("baz"))],
    ]).unwrap();

    let pid = PatternID::must(1);
    let group_index = 0;

    group_info.to_name(pid, group_index);
}

