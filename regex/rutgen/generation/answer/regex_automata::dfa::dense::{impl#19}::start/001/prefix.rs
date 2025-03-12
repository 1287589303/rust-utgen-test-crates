// Answer 0

#[test]
fn test_start_with_pattern_id_equal_to_length() {
    let table: Vec<u32> = vec![0; 16]; // sufficient size to avoid index panic
    let stride = 4;
    let kind = StartKind::Both;
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let pattern_len = Some(stride);
    let universal_start_unanchored = None;
    let universal_start_anchored = None;

    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let pid = PatternID(4.into()); // pid.as_usize() == len, where len = stride
    let anchored = Anchored::Pattern(pid);
    let start = Start::WordByte; // arbitrary choice

    let _result = start_table.start(anchored, start);
}

#[test]
fn test_start_with_pattern_id_exceeding_length() {
    let table: Vec<u32> = vec![0; 16]; // sufficient size to avoid index panic
    let stride = 4;
    let kind = StartKind::Both;
    let start_map = StartByteMap { map: [Start::NonWordByte; 256] };
    let pattern_len = Some(stride);
    let universal_start_unanchored = None;
    let universal_start_anchored = None;

    let start_table = StartTable {
        table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let pid = PatternID(5.into()); // pid.as_usize() exceeds len
    let anchored = Anchored::Pattern(pid);
    let start = Start::WordByte; // arbitrary choice

    let _result = start_table.start(anchored, start);
}

