// Answer 0

#[test]
fn test_build_hex_table_within_range() {
    let shift = 0;
    let table = build_hex_table(shift);
    assert_eq!(table[0], 0);
    assert_eq!(table[1], 1);
    assert_eq!(table[2], 2);
    assert_eq!(table[3], 3);
    assert_eq!(table[10], 10);
    assert_eq!(table[15], 15);
    assert_eq!(table[16], 16);
    assert_eq!(table[255], -1);
}

#[test]
fn test_build_hex_table_boundary_case() {
    let shift = 4;
    let table = build_hex_table(shift);
    assert_eq!(table[0], 0 << 4);
    assert_eq!(table[1], 1 << 4);
    assert_eq!(table[2], 2 << 4);
    assert_eq!(table[3], 3 << 4);
    assert_eq!(table[10], 10 << 4);
    assert_eq!(table[15], 15 << 4);
    assert_eq!(table[16], 16 << 4);
    assert_eq!(table[255], -1 << 4);
}

#[test]
fn test_build_hex_table_at_ch_limit() {
    let shift = 0;
    let table = build_hex_table(shift);
    assert_eq!(table[256], -1);
}

