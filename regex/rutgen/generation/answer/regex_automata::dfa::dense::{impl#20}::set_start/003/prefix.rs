// Answer 0

#[test]
fn test_set_start_with_anchored_yes_non_word_byte() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::NonWordByte;
    let id = StateID(0);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_anchored_yes_word_byte() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::WordByte; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::WordByte;
    let id = StateID(1);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_anchored_yes_text() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::Text; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::Text;
    let id = StateID(2);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_anchored_yes_line_lf() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::LineLF; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::LineLF;
    let id = StateID(3);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_anchored_yes_line_cr() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::LineCR; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::LineCR;
    let id = StateID(4);
    start_table.set_start(anchored, start, id);
}

#[test]
fn test_set_start_with_anchored_yes_custom_line_terminator() {
    let mut start_table = StartTable {
        table: vec![StateID(0); 8],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::CustomLineTerminator; 256] },
        stride: 1,
        pattern_len: Some(1),
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Yes;
    let start = Start::CustomLineTerminator;
    let id = StateID(5);
    start_table.set_start(anchored, start, id);
}

