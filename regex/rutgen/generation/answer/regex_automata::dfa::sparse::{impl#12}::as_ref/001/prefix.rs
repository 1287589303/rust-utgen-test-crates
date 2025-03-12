// Answer 0

#[test]
fn test_as_ref_both() {
    let table: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let kind = StartKind::Both;
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let stride = 10;
    let pattern_len = Some(5);
    let universal_start_unanchored = Some(StateID(1));
    let universal_start_anchored = Some(StateID(2));

    let start_table = StartTable {
        table: &table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };
    
    let _ = start_table.as_ref();
}

#[test]
fn test_as_ref_unanchored() {
    let table: [u8; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
    let kind = StartKind::Unanchored;
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let stride = 20;
    let pattern_len = Some(3);
    let universal_start_unanchored = Some(StateID(3));
    let universal_start_anchored = None;

    let start_table = StartTable {
        table: &table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let _ = start_table.as_ref();
}

#[test]
fn test_as_ref_anchored() {
    let table: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let kind = StartKind::Anchored;
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let stride = 5;
    let pattern_len = Some(0);
    let universal_start_unanchored = None;
    let universal_start_anchored = Some(StateID(4));

    let start_table = StartTable {
        table: &table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let _ = start_table.as_ref();
}

#[test]
fn test_as_ref_no_pattern_len() {
    let table: [u8; 8] = [10, 20, 30, 40, 50, 60, 70, 80];
    let kind = StartKind::Both;
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let stride = 15;
    let pattern_len = None;
    let universal_start_unanchored = Some(StateID(5));
    let universal_start_anchored = Some(StateID(6));

    let start_table = StartTable {
        table: &table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let _ = start_table.as_ref();
}

#[test]
fn test_as_ref_empty_table() {
    let table: [u8; 8] = [0; 8];
    let kind = StartKind::Unanchored;
    let start_map = StartByteMap { map: [Start::default(); 256] };
    let stride = 1;
    let pattern_len = Some(1);
    let universal_start_unanchored = None;
    let universal_start_anchored = None;

    let start_table = StartTable {
        table: &table,
        kind,
        start_map,
        stride,
        pattern_len,
        universal_start_unanchored,
        universal_start_anchored,
    };

    let _ = start_table.as_ref();
}

