// Answer 0

#[test]
fn test_read_varu32_single_byte() {
    let data = [0b0000_0001];
    let result = read_varu32(&data);
}

#[test]
fn test_read_varu32_another_single_byte() {
    let data = [0b0000_0011];
    let result = read_varu32(&data);
}

#[test]
fn test_read_varu32_max_single_byte() {
    let data = [0b0111_1111];
    let result = read_varu32(&data);
}

#[test]
fn test_read_varu32_min_multi_byte() {
    let data = [0b1000_0000, 0b0000_0001];
    let result = read_varu32(&data);
}

#[test]
fn test_read_varu32_max_multi_byte() {
    let data = [0b1111_1111, 0b0000_0001];
    let result = read_varu32(&data);
}

#[test]
fn test_read_varu32_multiple_bytes() {
    let data = [0b1000_0000, 0b0111_1111];
    let result = read_varu32(&data);
}

