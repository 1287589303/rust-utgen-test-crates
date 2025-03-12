// Answer 0

#[test]
fn test_start_valid_anchored() {
    let stride = 4;
    let pattern_len = Some(2);
    let start_table = StartTable {
        table: vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4)],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::from_usize(2).unwrap();
    
    let _result = start_table.start(anchored, start);
}

#[test]
fn test_start_valid_anchored_pattern() {
    let stride = 4;
    let pattern_len = Some(3);
    let start_table = StartTable {
        table: vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7)],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(1));
    let start = Start::from_usize(0).unwrap();

    let _result = start_table.start(anchored, start);
}

#[test]
fn test_start_panic_on_invalid_pattern() {
    let stride = 4;
    let pattern_len = Some(3);
    let start_table = StartTable {
        table: vec![StateID(0), StateID(1), StateID(2), StateID(3)],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(5));
    let start = Start::from_usize(1).unwrap();

    let _result = start_table.start(anchored, start);
}

