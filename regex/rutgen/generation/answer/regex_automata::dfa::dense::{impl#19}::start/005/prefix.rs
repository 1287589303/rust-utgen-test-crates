// Answer 0

#[test]
fn test_start_anchored_error_unanchored() {
    let table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Unanchored,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = table.start(Anchored::Yes, Start::Text);
}

#[test]
fn test_start_anchored_error_both() {
    let table = StartTable {
        table: vec![0; 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride: 4,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let result = table.start(Anchored::Yes, Start::WordByte);
}

