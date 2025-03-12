// Answer 0

#[test]
fn test_next_with_empty_index_to_name() {
    use alloc::vec;

    let group_info_inner = GroupInfoInner::default();
    let group_info = GroupInfo(Arc::new(group_info_inner));
    let pattern_id_iter = core::iter::empty::<PatternID>();
    let mut group_info_all_names = GroupInfoAllNames {
        group_info: &group_info,
        pids: pattern_id_iter,
        current_pid: None,
        names: None,
    };

    let result = group_info_all_names.next();

    assert!(result.is_none());
}

