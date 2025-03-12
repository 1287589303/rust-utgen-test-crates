// Answer 0

#[test]
fn test_set_start_no_anchored_non_word_byte() {
    let mut start_table = StartTable {
        table: vec![StateID::default(); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let state_id = StateID::default();
    start_table.set_start(Anchored::No, Start::NonWordByte, state_id);
}

#[test]
fn test_set_start_no_anchored_word_byte() {
    let mut start_table = StartTable {
        table: vec![StateID::default(); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::WordByte; 256] },
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let state_id = StateID::default();
    start_table.set_start(Anchored::No, Start::WordByte, state_id);
}

#[test]
fn test_set_start_no_anchored_text() {
    let mut start_table = StartTable {
        table: vec![StateID::default(); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride: 4,
        pattern_len: Some(2),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let state_id = StateID::default();
    start_table.set_start(Anchored::No, Start::Text, state_id);
}

#[test]
#[should_panic]
fn test_set_start_no_anchored_invalid_start() {
    let mut start_table = StartTable {
        table: vec![StateID::default(); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride: 4,
        pattern_len: None,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let state_id = StateID::default();
    start_table.set_start(Anchored::No, Start::NonWordByte, state_id);
}

