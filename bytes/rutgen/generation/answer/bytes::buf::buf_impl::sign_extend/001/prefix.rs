// Answer 0

#[test]
fn test_sign_extend_nbytes_1() {
    let val: u64 = 0b11111111; // 255 in decimal
    let nbytes: usize = 1;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_2() {
    let val: u64 = 0b1011111111111111; // 49151 in decimal
    let nbytes: usize = 2;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_3() {
    let val: u64 = 0b111111111111111100000000; // 0xFF00 in decimal
    let nbytes: usize = 3;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_4() {
    let val: u64 = 0b11111111111111111111111111111111; // 0xFFFFFFFF in decimal
    let nbytes: usize = 4;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_5() {
    let val: u64 = 0b1111111111111111111111111111111111111111; // 0xFFFFFFFFFF in decimal
    let nbytes: usize = 5;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_6() {
    let val: u64 = 0b11111111111111111111111111111111111111111111; // 0xFFFFFFFFFFFF in decimal
    let nbytes: usize = 6;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_7() {
    let val: u64 = 0b111111111111111111111111111111111111111111111111; // 0xFFFFFFFFFFFFFF in decimal
    let nbytes: usize = 7;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_8() {
    let val: u64 = 0b11111111111111111111111111111111111111111111111111111111; // 0xFFFFFFFFFFFFFFFF in decimal
    let nbytes: usize = 8;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_1_zero() {
    let val: u64 = 0; // Minimum value
    let nbytes: usize = 1;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_8_zero() {
    let val: u64 = 0; // Minimum value
    let nbytes: usize = 8;
    let result = sign_extend(val, nbytes);
}

#[test]
fn test_sign_extend_nbytes_8_edge() {
    let val: u64 = 0b10000000000000000000000000000000000000000000000000000000000000000; // Edge case for sign extension
    let nbytes: usize = 8;
    let result = sign_extend(val, nbytes);
}

