// Answer 0

#[test]
fn test_read_vari32_negative_one() {
    let data: &[u8] = &[0b1111_1111, 0b0000_0001]; // zig-zag encoding for -1
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_zero() {
    let data: &[u8] = &[0b0000_0000]; // zig-zag encoding for 0
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_one() {
    let data: &[u8] = &[0b0000_0001]; // zig-zag encoding for 1
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_negative127() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111]; // zig-zag encoding for -127
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_positive127() {
    let data: &[u8] = &[0b0111_1111]; // zig-zag encoding for 127
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_negative2147483648() {
    let data: &[u8] = &[0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111]; // zig-zag encoding for -2147483648
    let result = read_vari32(data);
}

#[test]
fn test_read_vari32_positive2147483647() {
    let data: &[u8] = &[0b0111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111]; // zig-zag encoding for 2147483647
    let result = read_vari32(data);
}

