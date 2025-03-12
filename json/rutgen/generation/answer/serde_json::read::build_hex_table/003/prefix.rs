// Answer 0

#[test]
fn test_build_hex_table_with_ch_equal_to_256_and_shift_zero() {
    let shift = 0;
    let result = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_with_ch_equal_to_256_and_shift_one() {
    let shift = 1;
    let result = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_with_ch_equal_to_256_and_shift_two() {
    let shift = 2;
    let result = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_with_ch_equal_to_256_and_shift_five() {
    let shift = 5;
    let result = build_hex_table(shift);
}

#[test]
fn test_build_hex_table_with_ch_equal_to_256_and_shift_ten() {
    let shift = 10;
    let result = build_hex_table(shift);
}

