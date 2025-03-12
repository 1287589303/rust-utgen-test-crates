// Answer 0

#[test]
fn test_len_case1() {
    let byte: u8 = 0b1000_0000; // 128
    let result = len(byte);
}

#[test]
fn test_len_case2() {
    let byte: u8 = 0b1000_0001; // 129
    let result = len(byte);
}

#[test]
fn test_len_case3() {
    let byte: u8 = 0b1000_0010; // 130
    let result = len(byte);
}

#[test]
fn test_len_case4() {
    let byte: u8 = 0b1000_1111; // 143
    let result = len(byte);
}

#[test]
fn test_len_case5() {
    let byte: u8 = 0b1101_1111; // 223
    let result = len(byte);
}

#[test]
fn test_len_case6() {
    let byte: u8 = 0b1110_0000; // 224
    let result = len(byte);
}

#[test]
fn test_len_case7() {
    let byte: u8 = 0b1111_1111; // 255
    let result = len(byte);
}

