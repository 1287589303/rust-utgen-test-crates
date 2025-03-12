// Answer 0

#[test]
fn test_set_start_with_pattern_0() {
    let stride = 1;
    let pattern_len = Some(1);
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(0));
    let start = Start::from_usize(0).unwrap();
    let id = StateID(1);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_pattern_1() {
    let stride = 1;
    let pattern_len = Some(2);
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(1));
    let start = Start::from_usize(2).unwrap();
    let id = StateID(2);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_pattern_boundary() {
    let stride = 2;
    let pattern_len = Some(2);
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(1));
    let start = Start::from_usize(3).unwrap();
    let id = StateID(3);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_multiple_patterns() {
    let stride = 2;
    let pattern_len = Some(3);
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(2));
    let start = Start::from_usize(4).unwrap();
    let id = StateID(4);
    start_table.set_start(anchored, start, id);
}

