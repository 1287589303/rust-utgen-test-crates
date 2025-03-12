// Answer 0

#[test]
fn test_ldh_mask_lower_a() {
    let b: u8 = b'a';
    let mut accu: u128 = 0u128;
    if !(b >= b'a' && b <= b'z' || b >= b'0' && b <= b'9' || b == b'-' || b == b'.') {
        accu |= 1u128 << b;
    }
}

#[test]
fn test_ldh_mask_lower_z() {
    let b: u8 = b'z';
    let mut accu: u128 = 0u128;
    if !(b >= b'a' && b <= b'z' || b >= b'0' && b <= b'9' || b == b'-' || b == b'.') {
        accu |= 1u128 << b;
    }
}

#[test]
fn test_ldh_mask_out_of_range() {
    let b: u8 = 128;
    let mut accu: u128 = 0u128;
    if !(b >= b'a' && b <= b'z' || b >= b'0' && b <= b'9' || b == b'-' || b == b'.') {
        accu |= 1u128 << b;
    }
}

