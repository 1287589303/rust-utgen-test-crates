// Answer 0

#[test]
fn test_build_hex_table_ch_less_than_256() {
    let shift = 0; // Example shift
    let _table = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_decode_hex_val_slow_none() {
    let shift = 0; // Example shift
    let mut table = [0; 256];
    for ch in 0..256 {
        table[ch] = match decode_hex_val_slow(ch as u8) {
            Some(_) => 0, // This won't be used since we're testing None cases
            None => -1,
        };
    }
    let _result = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_ch_equals_256() {
    let shift = 0; // Example shift
    let mut table = build_hex_table(shift);
    // Accessing table[256] would normally be an out-of-bounds access in Rust,
    // so here we are just ensuring compilation for this edge case.
    let _val = table[255]; // As the last valid index in Rust.
    let _val_out_of_bounds = table[256]; // This is to demonstrate the edge case, actual access will cause panic.
}

