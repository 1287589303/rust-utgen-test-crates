// Answer 0

#[test]
fn test_start_valid_pattern_0() {
    let stride = 8;
    let pattern_len = Some(5);
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7), StateID(8), StateID(9), StateID(10)];

    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let pid = PatternID(2);
    let result = start_table.start(Anchored::Pattern(pid), Start::from_usize(0).unwrap());
}

#[test]
fn test_start_valid_pattern_1() {
    let stride = 8;
    let pattern_len = Some(5);
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7), StateID(8), StateID(9), StateID(10)];

    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let pid = PatternID(1);
    let result = start_table.start(Anchored::Pattern(pid), Start::from_usize(4).unwrap());
}

#[test]
fn test_start_valid_pattern_2() {
    let stride = 8;
    let pattern_len = Some(5);
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7), StateID(8), StateID(9), StateID(10)];

    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let pid = PatternID(4);
    let result = start_table.start(Anchored::Pattern(pid), Start::from_usize(2).unwrap());
}

