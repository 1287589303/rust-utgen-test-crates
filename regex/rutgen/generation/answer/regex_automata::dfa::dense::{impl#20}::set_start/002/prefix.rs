// Answer 0

#[test]
fn test_set_start_pattern_invalid_pid() {
    let pattern_len = Some(1); // len > 0
    let stride = 2;
    let mut start_table = StartTable {
        table: vec![StateID(0); 8 + stride * pattern_len.unwrap()],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(SmallIndex(1))); // pid == len

    start_table.set_start(anchored, Start::Text, StateID(SmallIndex(2)));
}

#[test]
#[should_panic]
fn test_set_start_pattern_invalid_pid_panic() {
    let pattern_len = Some(1); // len > 0
    let stride = 2;
    let mut start_table = StartTable {
        table: vec![StateID(0); 8 + stride * pattern_len.unwrap()],
        kind: StartKind::Both,
        start_map: StartByteMap { map: [Start::NonWordByte; 256] },
        stride,
        pattern_len,
        universal_start_unanchored: None,
        universal_start_anchored: None,
    };
    let anchored = Anchored::Pattern(PatternID(SmallIndex(1))); // pid == len

    start_table.set_start(anchored, Start::Text, StateID(SmallIndex(2)));
}

