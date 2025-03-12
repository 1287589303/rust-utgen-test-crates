// Answer 0

#[test]
fn test_format_u8_n_equal_100() {
    let mut out = [0; 3];
    let result = format_u8(100, &mut out);
}

#[test]
fn test_format_u8_n_equal_101() {
    let mut out = [0; 3];
    let result = format_u8(101, &mut out);
}

#[test]
fn test_format_u8_n_equal_255() {
    let mut out = [0; 3];
    let result = format_u8(255, &mut out);
}

