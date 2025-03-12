// Answer 0

#[test]
fn test_write_varu32_min_boundary() {
    let mut data = Vec::new();
    write_varu32(&mut data, 0);
}

#[test]
fn test_write_varu32_low_byte() {
    let mut data = Vec::new();
    write_varu32(&mut data, 127);
}

#[test]
fn test_write_varu32_first_varint() {
    let mut data = Vec::new();
    write_varu32(&mut data, 128);
}

#[test]
fn test_write_varu32_mid_range() {
    let mut data = Vec::new();
    write_varu32(&mut data, 16383);
}

#[test]
fn test_write_varu32_next_boundary() {
    let mut data = Vec::new();
    write_varu32(&mut data, 16384);
}

#[test]
fn test_write_varu32_max_value() {
    let mut data = Vec::new();
    write_varu32(&mut data, 2147483647);
}

