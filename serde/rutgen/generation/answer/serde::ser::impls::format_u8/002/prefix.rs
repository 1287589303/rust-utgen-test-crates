// Answer 0

#[test]
fn test_format_u8_n_equals_10() {
    let mut out: [u8; 2] = [0; 2];
    let result = format_u8(10, &mut out);
}

#[test]
fn test_format_u8_n_equals_11() {
    let mut out: [u8; 2] = [0; 2];
    let result = format_u8(11, &mut out);
}

#[test]
fn test_format_u8_n_equals_50() {
    let mut out: [u8; 2] = [0; 2];
    let result = format_u8(50, &mut out);
}

#[test]
fn test_format_u8_n_equals_99() {
    let mut out: [u8; 2] = [0; 2];
    let result = format_u8(99, &mut out);
}

