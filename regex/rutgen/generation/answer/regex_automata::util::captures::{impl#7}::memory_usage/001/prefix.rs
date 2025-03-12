// Answer 0

#[test]
fn test_memory_usage_empty() {
    let group_info = GroupInfo::empty();
    let usage = group_info.memory_usage();
}

#[test]
fn test_memory_usage_with_slot_ranges() {
    let slot_ranges = vec![(SmallIndex(0), SmallIndex(1)), (SmallIndex(2), SmallIndex(3))];
    let group_info_inner = GroupInfoInner {
        slot_ranges,
        ..Default::default()
    };
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let usage = group_info.memory_usage();
}

#[test]
fn test_memory_usage_with_capture_name_map() {
    let mut name_to_index = CaptureNameMap::new();
    name_to_index.insert(Arc::from("test".to_string()), SmallIndex(1));
    let group_info_inner = GroupInfoInner {
        name_to_index: vec![name_to_index],
        ..Default::default()
    };
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let usage = group_info.memory_usage();
}

#[test]
fn test_memory_usage_with_index_to_name() {
    let index_to_name = vec![Some(Arc::from("group1".to_string())), None];
    let group_info_inner = GroupInfoInner {
        index_to_name: vec![index_to_name],
        ..Default::default()
    };
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let usage = group_info.memory_usage();
}

#[test]
fn test_memory_usage_with_memory_extra() {
    let group_info_inner = GroupInfoInner {
        memory_extra: 128,
        ..Default::default()
    };
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let usage = group_info.memory_usage();
}

#[test]
fn test_memory_usage_full() {
    let slot_ranges = vec![(SmallIndex(0), SmallIndex(1)), (SmallIndex(2), SmallIndex(3))];
    let mut name_to_index = CaptureNameMap::new();
    name_to_index.insert(Arc::from("group1".to_string()), SmallIndex(1));
    let index_to_name = vec![Some(Arc::from("group1".to_string())), None];
    let group_info_inner = GroupInfoInner {
        slot_ranges,
        name_to_index: vec![name_to_index],
        index_to_name: vec![index_to_name],
        memory_extra: 64,
    };
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let usage = group_info.memory_usage();
}

