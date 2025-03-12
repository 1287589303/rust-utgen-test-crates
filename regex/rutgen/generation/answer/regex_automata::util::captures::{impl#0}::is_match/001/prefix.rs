// Answer 0

#[test]
fn test_is_match_with_valid_pattern_id() {
    struct TestGroupInfo;

    let group_info = GroupInfo(Arc::new(TestGroupInfo));
    let captures = Captures {
        group_info,
        pid: Some(PatternID(SmallIndex(0))),
        slots: vec![None; 10],
    };
    
    let _ = captures.is_match();
}

#[test]
fn test_is_match_with_none_pattern_id() {
    struct TestGroupInfo;

    let group_info = GroupInfo(Arc::new(TestGroupInfo));
    let captures = Captures {
        group_info,
        pid: None,
        slots: vec![None; 10],
    };

    let _ = captures.is_match();
}

