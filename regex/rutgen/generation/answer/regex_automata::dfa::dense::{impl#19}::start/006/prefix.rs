// Answer 0

#[test]
fn test_start_unanchored_no() {
    let start = Start::from_usize(0).unwrap();
    let anchored = Anchored::No;
    
    let table = vec![StateID(0), StateID(1), StateID(2), StateID(3), StateID(4), StateID(5), StateID(6), StateID(7)];
    
    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let result = start_table.start(anchored, start);
}

#[test]
fn test_start_unanchored_nonzero() {
    let start = Start::from_usize(1).unwrap();
    let anchored = Anchored::No;

    let table = vec![StateID(8), StateID(9), StateID(10), StateID(11), StateID(12), StateID(13), StateID(14), StateID(15)];

    let start_table = StartTable {
        table,
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };

    let result = start_table.start(anchored, start);
}

